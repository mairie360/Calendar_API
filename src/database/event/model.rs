//! Types métier partagés par les vues de requête et les endpoints des événements.

use chrono::{DateTime, NaiveDate, Utc};
use mairie360_api_lib::database::db_interface::QueryParam;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Catégorie d'un événement : `meeting` (réunion), `activity` (activité), `ceremony` (cérémonie) ou `other` (autre, par défaut).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum EventCategory {
    Meeting,
    Activity,
    Ceremony,
    #[default]
    Other,
}

impl EventCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventCategory::Meeting => "meeting",
            EventCategory::Activity => "activity",
            EventCategory::Ceremony => "ceremony",
            EventCategory::Other => "other",
        }
    }
}

/// Visibilité d'un événement : `Public` (par défaut) ou `Private`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum EventVisibility {
    #[default]
    Public,
    Private,
}

impl EventVisibility {
    pub fn as_db(&self) -> &'static str {
        match self {
            EventVisibility::Public => "public",
            EventVisibility::Private => "private",
        }
    }
}

/// Unité de répétition d'un événement : `daily`, `weekly` ou `monthly`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
}

impl RecurrenceFrequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecurrenceFrequency::Daily => "daily",
            RecurrenceFrequency::Weekly => "weekly",
            RecurrenceFrequency::Monthly => "monthly",
        }
    }
}

pub const MAX_RECURRENCE_INTERVAL: u32 = 365;

/// Règle de répétition d'un événement, stockée dans `recurrence_rules`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct EventRecurrence {
    /// Unité de répétition, que `interval` multiplie.
    pub frequency: RecurrenceFrequency,
    /// Répétition tous les `interval` jours, semaines ou mois (1 à 365).
    #[schema(minimum = 1, maximum = 365, example = 1)]
    pub interval: u32,
    /// Jours d'une répétition hebdomadaire : 0 = dimanche … 6 = samedi. `null` hors
    /// répétition hebdomadaire.
    #[schema(example = json!([1, 4]))]
    pub days_of_week: Option<Vec<u8>>,
    /// Dernier jour (inclus) où l'événement se répète ; absent = sans fin.
    #[schema(value_type = Option<String>, format = Date, example = "2027-06-30")]
    pub ends_on: Option<NaiveDate>,
}

impl EventRecurrence {
    /// Vérifie la règle pour un événement qui commence à `start`.
    pub fn is_valid_for(&self, start: DateTime<Utc>) -> bool {
        let days_valid = self.days_of_week.as_ref().is_none_or(|days| {
            !days.is_empty()
                && days.len() <= 7
                && days.iter().all(|day| *day <= 6)
                && days.iter().collect::<std::collections::HashSet<_>>().len() == days.len()
        });
        (1..=MAX_RECURRENCE_INTERVAL).contains(&self.interval)
            && days_valid
            && self
                .ends_on
                .is_none_or(|ends_on| ends_on >= start.date_naive())
    }
}

/// Données complètes d'un événement, telles qu'écrites en base à la création ou à la modification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventInput {
    pub name: String,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub visibility: EventVisibility,
    pub category: EventCategory,
    pub service: Option<String>,
    pub location: Option<String>,
    pub recurrence: Option<EventRecurrence>,
}

impl EventInput {
    /// Paramètres `$2` à `$14` communs aux vues de création et de modification.
    pub fn query_params(&self) -> Vec<QueryParam> {
        let recurrence = self.recurrence.as_ref();
        vec![
            QueryParam::Text(self.name.clone()),
            QueryParam::Text(self.description.clone().unwrap_or_default()),
            QueryParam::DateTime(self.start),
            QueryParam::DateTime(self.end),
            QueryParam::Text(self.visibility.as_db().to_string()),
            QueryParam::Text(self.category.as_str().to_string()),
            QueryParam::Text(self.service.clone().unwrap_or_default()),
            QueryParam::Text(self.location.clone().unwrap_or_default()),
            QueryParam::Bool(recurrence.is_some()),
            QueryParam::Text(
                recurrence
                    .map(|r| r.frequency.as_str().to_string())
                    .unwrap_or_default(),
            ),
            QueryParam::I32(recurrence.map(|r| r.interval as i32).unwrap_or(1)),
            QueryParam::Text(
                recurrence
                    .and_then(|r| r.days_of_week.as_ref())
                    .map(|days| days.iter().map(u8::to_string).collect::<Vec<_>>().join(","))
                    .unwrap_or_default(),
            ),
            QueryParam::Text(
                recurrence
                    .and_then(|r| r.ends_on)
                    .map(|date| date.format("%Y-%m-%d").to_string())
                    .unwrap_or_default(),
            ),
        ]
    }
}

/// Colonnes de `recurrence_rules` dérivées des paramètres d'`EventInput` (`$4` début, `$5` fin, `$6` visibilité,
/// `$11` fréquence, `$12` intervalle, `$13` jours, `$14` dernier jour inclus → fin exclusive le lendemain à minuit UTC).
#[macro_export]
macro_rules! recurrence_rule_values_sql {
    () => {
        "NULLIF($11, '')::recurrence_type, $12::int, \
         string_to_array(NULLIF($13, ''), ',')::smallint[], $4::timestamptz, \
         ((NULLIF($14, '')::date + 1)::timestamp AT TIME ZONE 'UTC'), \
         ($4::timestamptz AT TIME ZONE 'UTC')::time, $5::timestamptz - $4::timestamptz, \
         NULLIF($6, '')::event_visibility"
    };
}

/// Règle de répétition d'un événement `e` joint à `recurrence_rules rr`, en JSON (`null` sans règle).
#[macro_export]
macro_rules! recurrence_json_sql {
    () => {
        "CASE WHEN rr.id IS NULL THEN NULL ELSE jsonb_build_object( \
            'frequency', rr.type_recurrence::text, \
            'interval', rr.intervalle, \
            'days_of_week', to_jsonb(rr.days_of_week), \
            'ends_on', to_char((rr.end_date AT TIME ZONE 'UTC') - interval '1 day', 'YYYY-MM-DD')) END"
    };
}

/// Vrai si l'événement `$1` doit être validé par un responsable : son créateur n'a que les rôles User ou
/// Guest, et un membre Responsable de l'événement partage un groupe avec lui.
#[macro_export]
macro_rules! event_requires_approval_sql {
    () => {
        "(EXISTS (SELECT 1 FROM events ev JOIN user_roles ur ON ur.user_id = ev.created_by \
                JOIN roles r ON r.id = ur.role_id WHERE ev.id = $1 AND r.name IN ('User', 'Guest')) \
          AND NOT EXISTS (SELECT 1 FROM events ev JOIN user_roles ur ON ur.user_id = ev.created_by \
                JOIN roles r ON r.id = ur.role_id \
                WHERE ev.id = $1 AND r.name IN ('Admin', 'Maire', 'Responsable')) \
          AND EXISTS (SELECT 1 FROM events ev \
                JOIN event_members m ON m.event_id = ev.id \
                JOIN user_roles ur ON ur.user_id = m.user_id \
                JOIN roles r ON r.id = ur.role_id AND r.name = 'Responsable' \
                JOIN group_members member_group ON member_group.user_id = m.user_id \
                JOIN group_members creator_group ON creator_group.group_id = member_group.group_id \
                    AND creator_group.user_id = ev.created_by \
                WHERE ev.id = $1))"
    };
}

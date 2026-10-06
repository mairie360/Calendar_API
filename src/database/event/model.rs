//! Types métier partagés par les vues de requête et les endpoints des événements.

use chrono::{DateTime, Datelike, NaiveDate, Utc};
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

/// Visibility of an event: `Public` (default) is readable by every authenticated user and listed in
/// everyone's calendar, `Private` only by its members and its creator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum EventVisibility {
    #[default]
    Public,
    Private,
}

impl EventVisibility {
    /// Reads `events.visibility`; anything but `private` is public.
    pub fn from_db(value: &str) -> Self {
        if value == "private" {
            EventVisibility::Private
        } else {
            EventVisibility::Public
        }
    }

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

/// Règle de répétition d'un événement, stockée dans `recurrence_rules`. An unknown field is a
/// `400`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
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
    /// Last day (inclusive) of the repetition, not before the start date and at most
    /// `2999-12-31`; absent = never ends.
    #[schema(value_type = Option<String>, format = Date, example = "2027-06-30")]
    pub ends_on: Option<NaiveDate>,
}

/// Earliest instant an event may start or end: `1970-01-01T00:00:00Z`.
pub const MIN_EVENT_DATE: DateTime<Utc> = DateTime::UNIX_EPOCH;

/// First year an event may no longer reach: dates stay before `3000-01-01T00:00:00Z`.
///
/// Postgres stores far wider ranges, but prints years outside 0001–9999 (`200000-…`, `… BC`) in a
/// form the reads cannot parse back, so an unbounded date made the event and every calendar
/// listing it answer `500` (MAIR-481). The window keeps a wide margin under year 10000 in any
/// time zone.
pub const MAX_EVENT_YEAR: i32 = 3000;

/// True when `date` is inside the window an event may use, `[1970-01-01, 3000-01-01)` UTC.
pub fn is_event_date(date: DateTime<Utc>) -> bool {
    date >= MIN_EVENT_DATE && date.year() < MAX_EVENT_YEAR
}

impl EventRecurrence {
    /// Checks the rule for an event starting at `start`: the faulty field and the rule it breaks,
    /// or `None` when the rule is valid.
    pub fn invalid_field(&self, start: DateTime<Utc>) -> Option<(&'static str, &'static str)> {
        let days_valid = self.days_of_week.as_ref().is_none_or(|days| {
            !days.is_empty()
                && days.len() <= 7
                && days.iter().all(|day| *day <= 6)
                && days.iter().collect::<std::collections::HashSet<_>>().len() == days.len()
        });
        if !(1..=MAX_RECURRENCE_INTERVAL).contains(&self.interval) {
            Some(("recurrence.interval", "must be between 1 and 365"))
        } else if !days_valid {
            Some((
                "recurrence.days_of_week",
                "must hold 1 to 7 distinct days between 0 (Sunday) and 6 (Saturday), or be null",
            ))
        } else if self
            .ends_on
            .is_some_and(|ends_on| ends_on < start.date_naive() || ends_on.year() >= MAX_EVENT_YEAR)
        {
            Some((
                "recurrence.ends_on",
                "must not be before the start date nor after 2999-12-31",
            ))
        } else {
            None
        }
    }

    /// True when the rule is valid for an event starting at `start`.
    pub fn is_valid_for(&self, start: DateTime<Utc>) -> bool {
        self.invalid_field(start).is_none()
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
    /// True when `self` moves the event compared to `other`: dates, recurrence or location. Such a
    /// change sends an approved or refused event back to `pending`.
    pub fn changes_schedule(&self, other: &EventInput) -> bool {
        self.start != other.start
            || self.end != other.end
            || self.recurrence != other.recurrence
            || self.location != other.location
    }

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
            QueryParam::I32(
                recurrence.map_or(1, |r| i32::try_from(r.interval).unwrap_or(i32::MAX)),
            ),
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

/// True when the creator given as an SQL expression (e.g. `"$1"`) needs a Responsable's approval
/// for their events: they only have the User or Guest roles.
#[macro_export]
macro_rules! creator_requires_approval_sql {
    ($creator:literal) => {
        concat!(
            "(EXISTS (SELECT 1 FROM user_roles ur JOIN roles r ON r.id = ur.role_id \
                WHERE ur.user_id = ",
            $creator,
            " AND r.name IN ('User', 'Guest')) \
              AND NOT EXISTS (SELECT 1 FROM user_roles ur JOIN roles r ON r.id = ur.role_id \
                WHERE ur.user_id = ",
            $creator,
            " AND r.name IN ('Admin', 'Maire', 'Responsable')))"
        )
    };
}

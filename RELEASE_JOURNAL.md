# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "347766b3542a94365b6bbedb390f3c8c4f974d50", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-10-21 03:51 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `347766b` | 2026-10-21 03:51 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Changements de comportement

- Les stacks Docker Compose (`docker-compose.yml`, `docker-compose-integration.yml`, `docker-compose-performance.yml`, `docker-compose-security.yml`) connectent désormais le service `calendar` avec le rôle Postgres `calendar_api` (mot de passe `calendar-api-password`) au lieu du superuser `postgres`. Une requête hors des grants de ce rôle échoue donc en local, comme en staging. Liquibase reçoit `-Dcalendar_api_password`. Postgres, Liquibase et le seeder gardent le superuser. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))

#### Technique

- CI/CD : `cicd.yml` et les nouveaux `promote-staging.yml` / `promote-prod.yml` pointent sur mairie360/CICD v4.4.1 ; la promotion staging et prod devient un déclenchement manuel (`workflow_dispatch`), et un push sur `main` ne va plus qu'en dev. Renovate regroupe les pins CICD des trois fichiers. (MAIR-494, MAIR-571, MAIR-612, PR [#181](https://github.com/mairie360/Calendar_API/pull/181), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#188](https://github.com/mairie360/Calendar_API/pull/188), [#191](https://github.com/mairie360/Calendar_API/pull/191), [#194](https://github.com/mairie360/Calendar_API/pull/194))
- Dépendances et tests : `tokio` 1.54.0 (PR [#186](https://github.com/mairie360/Calendar_API/pull/186)), `utoipa-swagger-ui` 10.0.2 (PR [#189](https://github.com/mairie360/Calendar_API/pull/189)), `mairie360_api_lib` 3.4.0 → 4.2.0, `rustls-webpki` et `webpki-roots` relevés, trois advisories RUSTSEC-2026-0367/0368/0369 ignorées dans `.cargo/audit.toml`. Les tests d'intégration passent par `common::api_db()` (rôle `calendar_api`) et ajoutent `tests/queries/db_role.rs`. Le diff de la PR [#180](https://github.com/mairie360/Calendar_API/pull/180) est tronqué, les autres fichiers de test ne sont pas vérifiés.

## Différence dev → staging (en dev, pas encore en staging)

#### Technique

- Les trois workflows (`cicd.yml`, `promote-staging.yml`, `promote-prod.yml`) passent de mairie360/CICD v4.4.0 à v4.4.1, ce qui corrige le dry run semantic-release de la promotion prod (MAIR-612, PR [#194](https://github.com/mairie360/Calendar_API/pull/194)). Aucun effet sur le service lui-même.

## Différence staging → prod (en staging, pas encore en prod)

#### Changements de comportement

- Les stacks Docker Compose connectent le service `calendar` avec le rôle Postgres `calendar_api` (mot de passe `calendar-api-password`) au lieu du superuser, et Liquibase reçoit `-Dcalendar_api_password`. Une requête hors des grants de ce rôle échoue en local. Le diff de cette repo ne montre pas la configuration de déploiement staging ou prod, donc l'effet réel sur ces environnements dépend de celle-ci. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))

#### Technique

- CI/CD : ajout des workflows manuels `promote-staging.yml` et `promote-prod.yml` (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191)), CICD relevé en v4.3.2, v4.3.1, puis v4.4.0 (PR [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#188](https://github.com/mairie360/Calendar_API/pull/188), [#191](https://github.com/mairie360/Calendar_API/pull/191)) ; le journal des releases rédigé par Claude arrive avec v4.3.1 (MAIR-571, [#184](https://github.com/mairie360/Calendar_API/pull/184)).
- Dépendances et tests : `tokio` 1.54.0 (PR [#186](https://github.com/mairie360/Calendar_API/pull/186)), `utoipa-swagger-ui` 10.0.2 (PR [#189](https://github.com/mairie360/Calendar_API/pull/189)), `mairie360_api_lib` 4.2.0, advisories RUSTSEC-2026-0367/0368/0369 ignorées dans `.cargo/audit.toml`, nouveau test `tests/queries/db_role.rs`. Le diff de la PR [#180](https://github.com/mairie360/Calendar_API/pull/180) est tronqué.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-21 03:51 UTC | dev | — | `347766b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39540184692) |
| 2026-10-21 03:20 UTC | staging | — | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39537392251) |
| 2026-10-21 01:44 UTC | dev | — | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39527785266) |
| 2026-10-20 19:02 UTC | dev | — | `47e7356` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39478871912) |
| 2026-10-20 17:07 UTC | staging | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 20:09 UTC | dev | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 19:35 UTC | dev | — | `31022c6` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307846830) |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->

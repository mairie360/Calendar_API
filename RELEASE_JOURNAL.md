# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "3b4668bc630095c53f5734f0ae590918f0bc3d81", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "updated": {"dev": "2026-10-19 20:09 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `a40acfd` | — |
| dev | — | `3b4668b` | 2026-10-19 20:09 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Les tests et les stacks `docker-compose*.yml` se connectent désormais à Postgres avec le rôle `calendar_api` (mot de passe `calendar-api-password`, passé à Liquibase via `-Dcalendar_api_password`) au lieu du superuser. Une requête non couverte par les grants échoue donc avec `42501`, comme en staging ; un test de régression `tests/queries/db_role.rs` le vérifie. Dans la même PR : `mairie360_api_lib` passe de 3.4.0 à 4.2.0, sans effet décrit sur Calendar, et les avis RUSTSEC-2026-0367/0368/0369 (`zip`, dépendance de build) sont ignorés dans l'audit. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 5 mises à jour de dépendances et de CI : workflow `mairie360/CICD` de v4.2.1 à v4.3.1, qui ajoute l'étape de journal des releases, et `tokio` de 1.53.3 à 1.54.0. (MAIR-571, [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#186](https://github.com/mairie360/Calendar_API/pull/186))

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Les tests et les stacks `docker-compose*.yml` se connectent désormais à Postgres avec le rôle `calendar_api` (mot de passe `calendar-api-password`, passé à Liquibase via `-Dcalendar_api_password`) au lieu du superuser. Une requête non couverte par les grants échoue donc avec `42501`, comme en staging ; un test de régression `tests/queries/db_role.rs` le vérifie. Dans la même PR : `mairie360_api_lib` passe de 3.4.0 à 4.2.0, sans effet décrit sur Calendar, et les avis RUSTSEC-2026-0367/0368/0369 (`zip`, dépendance de build) sont ignorés dans l'audit. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 5 mises à jour de dépendances et de CI : workflow `mairie360/CICD` de v4.2.1 à v4.3.1, qui ajoute l'étape de journal des releases, et `tokio` de 1.53.3 à 1.54.0. (MAIR-571, [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#186](https://github.com/mairie360/Calendar_API/pull/186))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-19 20:09 UTC | dev | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 19:35 UTC | dev | — | `31022c6` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307846830) |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->

# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "58ba9955863d72cd9aa053da7fd74689208979bd", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "updated": {"dev": "2026-10-19 18:38 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `a40acfd` | — |
| dev | — | `58ba995` | 2026-10-19 18:38 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Les tests exécutent désormais le code de Calendar avec le rôle Postgres `calendar_api` au lieu du superuser, comme en staging ; les fixtures gardent le superuser. `mairie360_api_lib` passe de 3.4.0 à 4.2.0. Le nouveau test de régression `tests/queries/db_role.rs` vérifie que `current_user` est `calendar_api` et que `SELECT … FROM projects` est refusé avec SQLSTATE `42501`. Dans `docker-compose*.yml`, le service `calendar` se connecte en `calendar_api`. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 2 mises à jour de dépendances : `mairie360/CICD` passe de v4.2.1 à v4.3.0. (MAIR-571, [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185))

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Les tests exécutent désormais le code de Calendar avec le rôle Postgres `calendar_api` au lieu du superuser, comme en staging ; les fixtures gardent le superuser. `mairie360_api_lib` passe de 3.4.0 à 4.2.0. Le nouveau test de régression `tests/queries/db_role.rs` vérifie que `current_user` est `calendar_api` et que `SELECT … FROM projects` est refusé avec SQLSTATE `42501`. Dans `docker-compose*.yml`, le service `calendar` se connecte en `calendar_api`. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 2 mises à jour de dépendances : `mairie360/CICD` passe de v4.2.1 à v4.3.0. (MAIR-571, [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->

# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "aeb7df22a34cf1354541dc5dea5d05d56ea6d203", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "updated": {"dev": "2026-10-19 19:28 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `a40acfd` | — |
| dev | — | `aeb7df2` | 2026-10-19 19:28 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Les tests et les stacks `docker-compose*.yml` exécutent désormais le code de Calendar avec le rôle Postgres `calendar_api`, comme en staging, et non plus avec le superuser : une requête hors des grants de `api_grants.sql` échoue dès les tests (`42501`). Le changement s'accompagne d'un test de non-régression `tests/queries/db_role.rs`, de la montée de `mairie360_api_lib` de 3.4.0 à 4.2.0 sans effet annoncé sur l'API, et de l'ignorance des avis `RUSTSEC-2026-0367` à `0369`, qui concernent la dépendance de build `zip`. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 3 mises à jour du workflow CI `mairie360/CICD`, de v4.2.1 à v4.3.1. ([#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187))

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Les tests et les stacks `docker-compose*.yml` exécutent désormais le code de Calendar avec le rôle Postgres `calendar_api`, comme en staging, et non plus avec le superuser : une requête hors des grants de `api_grants.sql` échoue dès les tests (`42501`). Le changement s'accompagne d'un test de non-régression `tests/queries/db_role.rs`, de la montée de `mairie360_api_lib` de 3.4.0 à 4.2.0 sans effet annoncé sur l'API, et de l'ignorance des avis `RUSTSEC-2026-0367` à `0369`, qui concernent la dépendance de build `zip`. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 3 mises à jour du workflow CI `mairie360/CICD`, de v4.2.1 à v4.3.1. ([#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->

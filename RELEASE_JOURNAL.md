# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "3b4668bc630095c53f5734f0ae590918f0bc3d81", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "3b4668bc630095c53f5734f0ae590918f0bc3d81", "updated": {"dev": "2026-10-19 20:09 UTC", "staging": "2026-10-20 17:07 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `3b4668b` | 2026-10-20 17:07 UTC |
| dev | — | `3b4668b` | 2026-10-19 20:09 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Aucun changement fonctionnel. Les tests et les stacks `docker-compose*.yml` se connectent désormais à Postgres avec le rôle `calendar_api`, comme en staging, au lieu du superuser. Un test de non-régression, `tests/queries/db_role.rs`, vérifie que le code testé tourne sous `calendar_api` et respecte ses grants (refus `42501` sur `projects`). La PR fait aussi passer `mairie360_api_lib` de 3.4.0 à 4.2.0 et ignore les avis `RUSTSEC-2026-0367`, `RUSTSEC-2026-0368` et `RUSTSEC-2026-0369` (zip, dépendance de build de `utoipa-swagger-ui`). D'après la PR, les ruptures de la 4.x ne touchent pas Calendar. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 5 mises à jour de la CI et des dépendances : le workflow `mairie360/CICD` passe de v4.2.1 à v4.3.1 et ajoute l'étape « Release journal » ; `tokio` passe de 1.53.3 à 1.54.0. (MAIR-571, [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#186](https://github.com/mairie360/Calendar_API/pull/186))

## Différence dev → staging (en dev, pas encore en staging)

_Rien._

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Aucun changement fonctionnel. Les tests et les stacks `docker-compose*.yml` se connectent désormais à Postgres avec le rôle `calendar_api`, comme en staging, au lieu du superuser. Un test de non-régression, `tests/queries/db_role.rs`, vérifie que le code testé tourne sous `calendar_api` et respecte ses grants (refus `42501` sur `projects`). La PR fait aussi passer `mairie360_api_lib` de 3.4.0 à 4.2.0 et ignore les avis `RUSTSEC-2026-0367`, `RUSTSEC-2026-0368` et `RUSTSEC-2026-0369` (zip, dépendance de build de `utoipa-swagger-ui`). D'après la PR, les ruptures de la 4.x ne touchent pas Calendar. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- 5 mises à jour de la CI et des dépendances : le workflow `mairie360/CICD` passe de v4.2.1 à v4.3.1 et ajoute l'étape « Release journal » ; `tokio` passe de 1.53.3 à 1.54.0. (MAIR-571, [#181](https://github.com/mairie360/Calendar_API/pull/181), [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#186](https://github.com/mairie360/Calendar_API/pull/186))

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-20 17:07 UTC | staging | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 20:09 UTC | dev | — | `3b4668b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39315820782) |
| 2026-10-19 19:35 UTC | dev | — | `31022c6` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307846830) |
| 2026-10-19 19:28 UTC | dev | — | `aeb7df2` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39307831941) |
| 2026-10-19 18:38 UTC | dev | — | `58ba995` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39300959456) |
<!-- release-journal-history:end -->

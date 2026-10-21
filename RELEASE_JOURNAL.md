# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "fe12f65c487b89f664da3d4c0422fa4768d177ed", "prod": "a40acfd5a1afcf946ac60c1dbe32fbe210bb8263", "prod_tag": "v1.1.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-10-21 08:29 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.1.0 | `a40acfd` | — |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `fe12f65` | 2026-10-21 08:29 UTC |

## Ajouté sur main depuis la dernière release prod (v1.1.0)

#### Technique
- Les stacks docker-compose (`docker-compose.yml`, `docker-compose-integration.yml`, `docker-compose-performance.yml`, `docker-compose-security.yml`) connectent désormais `calendar` avec le rôle Postgres `calendar_api` au lieu du superuser, et Liquibase reçoit `-Dcalendar_api_password`. Effet limité aux stacks locales et de test : une requête hors des grants du rôle y échoue en `42501`. Les fixtures de test gardent le superuser, et `tests/queries/db_role.rs` vérifie le rôle. Le diff est tronqué, donc le détail des tests reste à vérifier. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Bibliothèque `mairie360_api_lib` passée de 3.4.0 à 4.2.0 (dépendances et dev-dependencies, features inchangées). (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Ajout d'exceptions `cargo audit` dans `.cargo/audit.toml` pour RUSTSEC-2026-0367, 0368 et 0369 (zip 8, build-dependency de utoipa-swagger-ui). (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Mises à jour de dépendances : utoipa-swagger-ui 10.0.2 ([#189](https://github.com/mairie360/Calendar_API/pull/189)), tokio 1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186)) et image Docker de développement `rust:1.99-bookworm` épinglée au digest 5c1a173 ([#197](https://github.com/mairie360/Calendar_API/pull/197)).
- CI : workflows de promotion manuelle `promote-staging.yml` et `promote-prod.yml` ajoutés, et `renovate.json` couvre les trois fichiers (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191)). Le pin mairie360/CICD passe de v4.2.2 à v4.3.0, v4.3.1, v4.3.2 puis v4.4.0 (MAIR-571 et renovate, [#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#188](https://github.com/mairie360/Calendar_API/pull/188), [#184](https://github.com/mairie360/Calendar_API/pull/184), [#181](https://github.com/mairie360/Calendar_API/pull/181)). Le journal de release est rédigé dans `RELEASE_JOURNAL.md` par le workflow CICD, sans effet sur le service.
- CI : pins mairie360/CICD v4.4.1 puis v4.4.2 dans `cicd.yml` et les workflows de promotion, et mise à jour de `CLAUDE.md` sur la promotion (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196)).

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Image Docker de développement `rust:1.99-bookworm` épinglée au digest 5c1a173 (`development.Dockerfile`), sans effet sur le service. ([#197](https://github.com/mairie360/Calendar_API/pull/197))
- Pins mairie360/CICD passés à v4.4.1 puis v4.4.2 dans `cicd.yml`, `promote-staging.yml` et `promote-prod.yml`, sans changement fonctionnel côté service. (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196))

## Différence staging → prod (en staging, pas encore en prod)

#### Technique
- Workflows de promotion manuelle vers staging et prod (`promote-staging.yml`, `promote-prod.yml`), `renovate.json` et documentation de promotion dans `CLAUDE.md`, sans changement pour le service. (MAIR-494, [#191](https://github.com/mairie360/Calendar_API/pull/191))
- Pins mairie360/CICD v4.3.0, v4.3.1, v4.3.2 et v4.4.0 dans les workflows, avec l'étape de journal de release MAIR-571 ajoutée à CICD (sans effet sur le service). ([#185](https://github.com/mairie360/Calendar_API/pull/185), [#187](https://github.com/mairie360/Calendar_API/pull/187), [#188](https://github.com/mairie360/Calendar_API/pull/188), [#184](https://github.com/mairie360/Calendar_API/pull/184))
- Mises à jour de dépendances : utoipa-swagger-ui 10.0.2 ([#189](https://github.com/mairie360/Calendar_API/pull/189)) et tokio 1.54.0 ([#186](https://github.com/mairie360/Calendar_API/pull/186)). Le diff de ces deux PR est vide.
- Tests : les tests d'API passent par le rôle `calendar_api` au lieu du superuser, et `mairie360_api_lib` passe de 3.4.0 à 4.2.0. Le diff de ce commit est tronqué, donc le détail reste à vérifier. (MAIR-556, [#180](https://github.com/mairie360/Calendar_API/pull/180))
- Mise à jour de dépendances : pin mairie360/CICD v4.2.2 ([#181](https://github.com/mairie360/Calendar_API/pull/181)), sans effet sur le service.

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-10-21 08:29 UTC | dev | — | `fe12f65` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39568348176) |
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

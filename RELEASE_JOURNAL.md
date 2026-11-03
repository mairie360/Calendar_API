# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "20301224aa02b57aa12a3a4fa36b844553358783", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-11-03 00:18 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `2030122` | 2026-11-03 00:18 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

#### Changements de comportement
- Les journaux des requêtes n'écrivent plus la query string ni l'en-tête `Referer`. Avant, la ligne de requête complète était journalisée (ex. `GET /api/v1/calendar?start=...`). Maintenant ne restent que l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Épinglage par digest des images Docker des fichiers compose (postgres, redis, nginx, node, curl, k6, newman, ZAP) et de `development.Dockerfile` (`rust:1.99-bookworm`), mise à jour de `tokio` en 1.54.1. ([#210](https://github.com/mairie360/Calendar_API/pull/210), [#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203), [#200](https://github.com/mairie360/Calendar_API/pull/200))
- Image runtime de l'API passée de distroless Debian 12 à Debian 13 (builder `rust:1.99-slim-trixie`, `development.Dockerfile` en trixie), test `mair_741_every_image_runs_on_debian_13`. (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Ajout de `access-matrix.yaml`, de son test d'accès et de la dépendance `yaml_serde` (dev) ; aucun changement de route. (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161))
- Marqueur RGPD de logs : stack `docker-compose-gdpr-marker.yml`, `gdpr-marker.yaml`, `gdpr_marker_test.sh`, tests de logs ; ajout de la dépendance `log` en dev. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))
- Checklist RGPD automatique sur les PR (workflow `gdpr-pr.yml`). (MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157))
- Workflows CI/CD montés sur mairie360/CICD v4.4.1 à v4.4.3 (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199)), puis en v5.2.2 avec `ANTHROPIC_WORKSPACE_ID` transmis à `promote-prod` et test de cohérence des versions CICD. (MAIR-771, [#212](https://github.com/mairie360/Calendar_API/pull/212))

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- Les journaux des requêtes n'écrivent plus la query string ni l'en-tête `Referer`. Avant, la ligne de requête complète était journalisée (ex. `GET /api/v1/calendar?start=...`). Maintenant ne restent que l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Épinglage par digest des images Docker des fichiers compose (postgres, redis, nginx, node, curl, k6, newman, ZAP) et de `development.Dockerfile` (`rust:1.99-bookworm`), mise à jour de `tokio` en 1.54.1. ([#210](https://github.com/mairie360/Calendar_API/pull/210), [#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203), [#200](https://github.com/mairie360/Calendar_API/pull/200))
- Image runtime de l'API passée de distroless Debian 12 à Debian 13 (builder `rust:1.99-slim-trixie`, `development.Dockerfile` en trixie), test `mair_741_every_image_runs_on_debian_13`. (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Ajout de `access-matrix.yaml`, de son test d'accès et de la dépendance `yaml_serde` (dev) ; aucun changement de route. (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161))
- Marqueur RGPD de logs : stack `docker-compose-gdpr-marker.yml`, `gdpr-marker.yaml`, `gdpr_marker_test.sh`, tests de logs ; ajout de la dépendance `log` en dev. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))
- Checklist RGPD automatique sur les PR (workflow `gdpr-pr.yml`). (MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157))
- Workflows CI/CD montés sur mairie360/CICD v4.4.1 à v4.4.3 (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199)), puis en v5.2.2 avec `ANTHROPIC_WORKSPACE_ID` transmis à `promote-prod` et test de cohérence des versions CICD. (MAIR-771, [#212](https://github.com/mairie360/Calendar_API/pull/212))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-03 00:18 UTC | dev | — | `2030122` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41550514257) |
| 2026-11-01 12:22 UTC | dev | — | `f266efe` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41305127787) |
| 2026-10-22 23:06 UTC | dev | — | `564d98a` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39868682896) |
| 2026-10-22 05:42 UTC | dev | — | `f5d8257` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39728806480) |
| 2026-10-22 02:53 UTC | dev | — | `0e0506d` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39712216266) |
| 2026-10-21 09:11 UTC | prod | v1.2.0 | `dbc5e3f` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39573686812) |
| 2026-10-21 09:07 UTC | dev | — | `6460af9` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39573578570) |
| 2026-10-21 08:30 UTC | dev | — | `6a5953c` | [run](https://github.com/mairie360/Calendar_API/actions/runs/39568204180) |
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

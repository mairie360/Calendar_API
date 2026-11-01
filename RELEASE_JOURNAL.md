# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "f266efe17ea6713dfbddb89fc08dfd61692133e7", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-11-01 12:22 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `f266efe` | 2026-11-01 12:22 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

#### Corrections
- Image d'exécution de l'API : avant, `gcr.io/distroless/cc-debian12:nonroot` embarquait une `libssl3` non corrigée (CVE-2026-84782) ; maintenant, `gcr.io/distroless/cc-debian13:nonroot` avec un builder `rust:1.99-slim-trixie`, et 0 vulnérabilité HIGH/CRITICAL corrigeable au scan Trivy de la PR. (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Journaux des requêtes : avant, la ligne de requête complète (query string incluse, par exemple `GET /api/v1/calendar?start=<adresse>`) et le `Referer` étaient écrits ; maintenant seuls l'adresse du client, la méthode, le chemin sans query string, le statut, la taille et la durée sont journalisés. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Matrice d'accès : `access-matrix.yaml` décrit, pour chaque opération de l'OpenAPI, les rôles et relations autorisés, et `tests/endpoints/access_matrix.rs` vérifie que l'API répond conformément (ajout de la dépendance de dev `yaml_serde`), sans effet à l'exécution. (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161))
- Test du marqueur RGPD : ajout de `docker-compose-gdpr-marker.yml`, `gdpr-marker.yaml` et `gdpr_marker_test.sh`, test unitaire `request_log`, exception RUSTSEC-2026-0387 (crypto-bigint) dans `audit.toml`, et test `mair_741_every_image_runs_on_debian_13` sur les images des Dockerfiles. (MAIR-290, MAIR-741, [#153](https://github.com/mairie360/Calendar_API/pull/153), [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Checklist RGPD des PR : workflow `.github/workflows/gdpr-pr.yml` qui appelle le workflow réutilisable de mairie360/CICD. (MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157))
- Dépendances : `tokio` 1.54.0 → 1.54.1 ([#200](https://github.com/mairie360/Calendar_API/pull/200), diff absent) ; digests de `rust:1.99-bookworm` mis à jour par Renovate trois fois ([#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203)), puis remplacés par l'image trixie de [#208](https://github.com/mairie360/Calendar_API/pull/208).
- Pins CICD de mairie360/CICD : v4.4.0 → v4.4.1 → v4.4.2 → v4.4.3 dans les workflows de release et de promotion, pour corriger la création du tag et de la release en prod. (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199))

## Différence dev → staging (en dev, pas encore en staging)

#### Corrections
- Image d'exécution de l'API : avant, `gcr.io/distroless/cc-debian12:nonroot` embarquait une `libssl3` non corrigée (CVE-2026-84782) ; maintenant, `gcr.io/distroless/cc-debian13:nonroot` avec un builder `rust:1.99-slim-trixie`, et 0 vulnérabilité HIGH/CRITICAL corrigeable au scan Trivy de la PR. (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Journaux des requêtes : avant, la ligne de requête complète (query string incluse, par exemple `GET /api/v1/calendar?start=<adresse>`) et le `Referer` étaient écrits ; maintenant seuls l'adresse du client, la méthode, le chemin sans query string, le statut, la taille et la durée sont journalisés. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Matrice d'accès : `access-matrix.yaml` décrit, pour chaque opération de l'OpenAPI, les rôles et relations autorisés, et `tests/endpoints/access_matrix.rs` vérifie que l'API répond conformément (ajout de la dépendance de dev `yaml_serde`), sans effet à l'exécution. (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161))
- Test du marqueur RGPD : ajout de `docker-compose-gdpr-marker.yml`, `gdpr-marker.yaml` et `gdpr_marker_test.sh`, test unitaire `request_log`, exception RUSTSEC-2026-0387 (crypto-bigint) dans `audit.toml`, et test `mair_741_every_image_runs_on_debian_13` sur les images des Dockerfiles. (MAIR-290, MAIR-741, [#153](https://github.com/mairie360/Calendar_API/pull/153), [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Checklist RGPD des PR : workflow `.github/workflows/gdpr-pr.yml` qui appelle le workflow réutilisable de mairie360/CICD. (MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157))
- Dépendances : `tokio` 1.54.0 → 1.54.1 ([#200](https://github.com/mairie360/Calendar_API/pull/200), diff absent) ; digests de `rust:1.99-bookworm` mis à jour par Renovate trois fois ([#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203)), puis remplacés par l'image trixie de [#208](https://github.com/mairie360/Calendar_API/pull/208).
- Pins CICD de mairie360/CICD : v4.4.0 → v4.4.1 → v4.4.2 → v4.4.3 dans les workflows de release et de promotion, pour corriger la création du tag et de la release en prod. (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199))

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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

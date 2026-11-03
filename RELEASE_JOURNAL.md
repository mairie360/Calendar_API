# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "9c9da5b8e4cfc2490a534ebca7c14b71268e351a", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-11-03 01:35 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `9c9da5b` | 2026-11-03 01:35 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

#### Changements de comportement
- Les jetons HS256 sont désormais refusés (401) quand `JWT_SECRET` n'est pas défini : l'API ne lit plus que `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, jetons signés en `EdDSA`). Tant que `JWT_SECRET` est défini, la lib accepte encore les HS256 signés avec lui (transition) : un déploiement doit poser `JWT_PUBLIC_KEY` puis retirer `JWT_SECRET`. Un appelant (BFF, script) qui signe encore en HS256 avec le secret partagé reçoit désormais 401. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))
- Les logs de requêtes n'incluent plus la query string ni le `Referer` : ils ne gardent que l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Scripts de test de l'intégration, ZAP, k6 et du marqueur RGPD : `test_secrets.sh` génère une paire Ed25519 par run, signe les jetons des runners comme Core, et les fichiers compose exigent `JWT_PUBLIC_KEY` ; Newman et k6 reçoivent leurs jetons par variables ou fichier. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))
- Contrôles RGPD : `gdpr-pr.yml` (checklist RGPD sur chaque PR, MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157)), marqueur `gdpr-marker.yaml` et `gdpr_marker_test.sh` (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153)).
- Matrice d'accès `access-matrix.yaml` et test de couverture de l'OpenAPI `tests/endpoints/access_matrix.rs` (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161)) ; le diff visible ne change pas le code des droits.
- Image d'exécution passée de distroless Debian 12 à Debian 13, builder et `development.Dockerfile` alignés sur trixie (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208)).
- Workflows CI/CD épinglés sur mairie360/CICD v4.4.1, v4.4.2, v4.4.3 (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199)), puis v5.2.2 pour activer les contrôles RGPD (MAIR-771, [#212](https://github.com/mairie360/Calendar_API/pull/212)).
- Dépendances : digests Docker épinglés par Renovate (postgres, redis, nginx, node, curl, k6, newman, ZAP), image `rust:1.99-bookworm` mise à jour trois fois, et `tokio` 1.54.0 → 1.54.1 ([#210](https://github.com/mairie360/Calendar_API/pull/210), [#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203), [#200](https://github.com/mairie360/Calendar_API/pull/200)).

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- Les jetons HS256 sont désormais refusés (401) quand `JWT_SECRET` n'est pas défini : l'API ne lit plus que `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, jetons signés en `EdDSA`). Tant que `JWT_SECRET` est défini, la lib accepte encore les HS256 signés avec lui (transition) : un déploiement doit poser `JWT_PUBLIC_KEY` puis retirer `JWT_SECRET`. Un appelant (BFF, script) qui signe encore en HS256 avec le secret partagé reçoit désormais 401. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))
- Les logs de requêtes n'incluent plus la query string ni le `Referer` : ils ne gardent que l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Scripts de test de l'intégration, ZAP, k6 et du marqueur RGPD : `test_secrets.sh` génère une paire Ed25519 par run, signe les jetons des runners comme Core, et les fichiers compose exigent `JWT_PUBLIC_KEY` ; Newman et k6 reçoivent leurs jetons par variables ou fichier. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))
- Contrôles RGPD : `gdpr-pr.yml` (checklist RGPD sur chaque PR, MAIR-295, [#157](https://github.com/mairie360/Calendar_API/pull/157)), marqueur `gdpr-marker.yaml` et `gdpr_marker_test.sh` (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153)).
- Matrice d'accès `access-matrix.yaml` et test de couverture de l'OpenAPI `tests/endpoints/access_matrix.rs` (MAIR-288, [#161](https://github.com/mairie360/Calendar_API/pull/161)) ; le diff visible ne change pas le code des droits.
- Image d'exécution passée de distroless Debian 12 à Debian 13, builder et `development.Dockerfile` alignés sur trixie (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208)).
- Workflows CI/CD épinglés sur mairie360/CICD v4.4.1, v4.4.2, v4.4.3 (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199)), puis v5.2.2 pour activer les contrôles RGPD (MAIR-771, [#212](https://github.com/mairie360/Calendar_API/pull/212)).
- Dépendances : digests Docker épinglés par Renovate (postgres, redis, nginx, node, curl, k6, newman, ZAP), image `rust:1.99-bookworm` mise à jour trois fois, et `tokio` 1.54.0 → 1.54.1 ([#210](https://github.com/mairie360/Calendar_API/pull/210), [#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203), [#200](https://github.com/mairie360/Calendar_API/pull/200)).

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-03 01:35 UTC | dev | — | `9c9da5b` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41559210628) |
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

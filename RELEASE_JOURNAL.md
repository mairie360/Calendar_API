# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "30217fa8e5f352899486da6c93f99661036e0ef5", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-11-03 11:19 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `30217fa` | 2026-11-03 11:19 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

#### Changements de comportement
- L'API ne lit plus `JWT_SECRET` pour vérifier les jetons : elle utilise `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, au format PEM). Tant que `JWT_SECRET` est défini, les jetons HS256 signés avec lui sont encore acceptés (transition). Sans `JWT_SECRET`, tout jeton HS256 reçoit un 401 et seuls les jetons EdDSA signés par Core sont acceptés. Tout appelant qui signe encore en HS256 doit donc passer à la clé de Core. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))

#### Corrections
- Journaux des requêtes : avant, la ligne de requête complète (query string comprise, par exemple `GET /api/v1/calendar?start=<valeur saisie>`) et le `Referer` étaient écrits. Maintenant, seuls l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée sont journalisés. Les erreurs de `mairie360_api_lib` ne reprennent plus les valeurs reçues : SQLSTATE, contrainte, table et colonne remplacent le message Postgres. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Image d'exécution des APIs passée de distroless Debian 12 à Debian 13 (`cc-debian13:nonroot`), builder et image de développement sur trixie, pour corriger la CVE-2026-84782 dans libssl3. (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Dépendances : actix-web 4.17 → 4.18.1, log (dev) 0.4.34 → 0.4.35, tokio 1.54.0 → 1.54.1, mairie360_api_lib 4.2.0 → 4.3.0 ; images Docker épinglées par digest (Rust, postgres, redis, curl, k6, newman, nginx, node, ZAP). (#204, #200, #210, #203, #202, #197)
- Tests et CI : matrice d'accès `access-matrix.yaml` vérifiée contre l'OpenAPI (MAIR-288, #161), garde sur la version CICD v5.2.2 et les clés Anthropic (MAIR-771, #212), test du marqueur RGPD des journaux (MAIR-290, #153), checklist RGPD des PR (MAIR-295, #157), pins CICD v4.4.1 à v5.2.2 des workflows de release (#194, #196, #199). Aucun effet visible pour les utilisateurs.

## Différence dev → staging (en dev, pas encore en staging)

#### Changements de comportement
- L'API ne lit plus `JWT_SECRET` pour vérifier les jetons : elle utilise `JWT_PUBLIC_KEY` (clé publique Ed25519 de Core_API, au format PEM). Tant que `JWT_SECRET` est défini, les jetons HS256 signés avec lui sont encore acceptés (transition). Sans `JWT_SECRET`, tout jeton HS256 reçoit un 401 et seuls les jetons EdDSA signés par Core sont acceptés. Tout appelant qui signe encore en HS256 doit donc passer à la clé de Core. (MAIR-680, [#214](https://github.com/mairie360/Calendar_API/pull/214))

#### Corrections
- Journaux des requêtes : avant, la ligne de requête complète (query string comprise, par exemple `GET /api/v1/calendar?start=<valeur saisie>`) et le `Referer` étaient écrits. Maintenant, seuls l'adresse du client, la méthode, le chemin sans query, le statut, la taille et la durée sont journalisés. Les erreurs de `mairie360_api_lib` ne reprennent plus les valeurs reçues : SQLSTATE, contrainte, table et colonne remplacent le message Postgres. (MAIR-290, [#153](https://github.com/mairie360/Calendar_API/pull/153))

#### Technique
- Image d'exécution des APIs passée de distroless Debian 12 à Debian 13 (`cc-debian13:nonroot`), builder et image de développement sur trixie, pour corriger la CVE-2026-84782 dans libssl3. (MAIR-741, [#208](https://github.com/mairie360/Calendar_API/pull/208))
- Dépendances : actix-web 4.17 → 4.18.1, log (dev) 0.4.34 → 0.4.35, tokio 1.54.0 → 1.54.1, mairie360_api_lib 4.2.0 → 4.3.0 ; images Docker épinglées par digest (Rust, postgres, redis, curl, k6, newman, nginx, node, ZAP). (#204, #200, #210, #203, #202, #197)
- Tests et CI : matrice d'accès `access-matrix.yaml` vérifiée contre l'OpenAPI (MAIR-288, #161), garde sur la version CICD v5.2.2 et les clés Anthropic (MAIR-771, #212), test du marqueur RGPD des journaux (MAIR-290, #153), checklist RGPD des PR (MAIR-295, #157), pins CICD v4.4.1 à v5.2.2 des workflows de release (#194, #196, #199). Aucun effet visible pour les utilisateurs.

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
| 2026-11-03 11:19 UTC | dev | — | `30217fa` | [run](https://github.com/mairie360/Calendar_API/actions/runs/41624171679) |
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

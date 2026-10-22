# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "f5d82577178536aa794372dd70fd0b4965497c2a", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-10-22 05:42 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `f5d8257` | 2026-10-22 05:42 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

#### Technique
- Pipeline de release : `mairie360/CICD` passe de v4.4.0 à v4.4.3 dans `cicd.yml`, `promote-staging.yml` et `promote-prod.yml`. Ces commits corrigent la création de release et de tag en prod, sans effet sur le comportement de l'API. (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199))
- Mises à jour de dépendances : image `rust:1.99-bookworm` épinglée par digest dans `development.Dockerfile` (`e12566d` → `5c1a173`, puis `edaa2a8`, MAIR-427, [#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202)). La PR [#200](https://github.com/mairie360/Calendar_API/pull/200) annonce aussi tokio 1.54.0 → 1.54.1, mais le diff n'est pas fourni et n'a pas pu être vérifié.

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Pipeline de release : `mairie360/CICD` passe de v4.4.0 à v4.4.3 dans `cicd.yml`, `promote-staging.yml` et `promote-prod.yml`. Ces commits corrigent la création de release et de tag en prod, sans effet sur le comportement de l'API. (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194), [#196](https://github.com/mairie360/Calendar_API/pull/196), [#199](https://github.com/mairie360/Calendar_API/pull/199))
- Mises à jour de dépendances : image `rust:1.99-bookworm` épinglée par digest dans `development.Dockerfile` (`e12566d` → `5c1a173`, puis `edaa2a8`, MAIR-427, [#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202)). La PR [#200](https://github.com/mairie360/Calendar_API/pull/200) annonce aussi tokio 1.54.0 → 1.54.1, mais le diff n'est pas fourni et n'a pas pu être vérifié.

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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

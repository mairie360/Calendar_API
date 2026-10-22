# Journal des releases — Calendar_API

<!-- release-journal-state {"dev": "564d98ac6d393502f0b03ca10a53fc01abee7bbd", "prod": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "prod_tag": "v1.2.0", "staging": "dbc5e3f42b0e4c45c4a09f544955bd656afc879e", "updated": {"dev": "2026-10-22 23:06 UTC", "prod": "2026-10-21 09:11 UTC", "staging": "2026-10-21 03:20 UTC"}} -->
> Tenu à jour par la CI (`mairie360/CICD`, action `release/release-journal`) à chaque passage en dev, staging et prod. Ne pas modifier à la main : le fichier est réécrit au passage suivant.

| Environnement | Version | Commit | Mis à jour |
| --- | --- | --- | --- |
| prod | v1.2.0 | `dbc5e3f` | 2026-10-21 09:11 UTC |
| staging | — | `dbc5e3f` | 2026-10-21 03:20 UTC |
| dev | — | `564d98a` | 2026-10-22 23:06 UTC |

## Ajouté sur main depuis la dernière release prod (v1.2.0)

#### Technique
- Pipeline de release : `cicd.yml`, `promote-staging.yml` et `promote-prod.yml` passent de mairie360/CICD v4.4.0 à v4.4.1 (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194)), puis v4.4.2 (MAIR-612, [#196](https://github.com/mairie360/Calendar_API/pull/196)) et v4.4.3 (MAIR-612, [#199](https://github.com/mairie360/Calendar_API/pull/199)). Selon la description de la PR #199, la v4.4.3 corrige la création du tag et de la release dans `promote-prod`. Ces commits ne modifient pas le code de l'API.
- Dépendances : Renovate met à jour trois fois le digest de `rust:1.99-bookworm` dans `development.Dockerfile` ([#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203)) et met tokio à jour de 1.54.0 à 1.54.1 ([#200](https://github.com/mairie360/Calendar_API/pull/200)). Le diff de la PR #200 n'est pas fourni, donc le changement de `Cargo.toml` ou `Cargo.lock` n'a pas pu être vérifié. Le Dockerfile de production n'apparaît dans aucun de ces diffs.

## Différence dev → staging (en dev, pas encore en staging)

#### Technique
- Pipeline de release : `cicd.yml`, `promote-staging.yml` et `promote-prod.yml` passent de mairie360/CICD v4.4.0 à v4.4.1 (MAIR-612, [#194](https://github.com/mairie360/Calendar_API/pull/194)), puis v4.4.2 (MAIR-612, [#196](https://github.com/mairie360/Calendar_API/pull/196)) et v4.4.3 (MAIR-612, [#199](https://github.com/mairie360/Calendar_API/pull/199)). Selon la description de la PR #199, la v4.4.3 corrige la création du tag et de la release dans `promote-prod`. Ces commits ne modifient pas le code de l'API.
- Dépendances : Renovate met à jour trois fois le digest de `rust:1.99-bookworm` dans `development.Dockerfile` ([#197](https://github.com/mairie360/Calendar_API/pull/197), [#202](https://github.com/mairie360/Calendar_API/pull/202), [#203](https://github.com/mairie360/Calendar_API/pull/203)) et met tokio à jour de 1.54.0 à 1.54.1 ([#200](https://github.com/mairie360/Calendar_API/pull/200)). Le diff de la PR #200 n'est pas fourni, donc le changement de `Cargo.toml` ou `Cargo.lock` n'a pas pu être vérifié. Le Dockerfile de production n'apparaît dans aucun de ces diffs.

## Différence staging → prod (en staging, pas encore en prod)

_Rien._

## Historique des passages

<!-- release-journal-history:start -->
| Date | Environnement | Version | Commit | Run |
| --- | --- | --- | --- | --- |
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

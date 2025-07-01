# Github CLI

## Requirements
- Rust
- git
- Set up  GITHUB\_TOKEN  environment variable like this:
```bash
export GITHUB_TOKEN=your_token
```
- clone and build this project
```bash
git clone git@github.com:aragami3070/github-cli.git
cd github-cli
cargo build
```
- link binary to your favorite bin directory
```bash
ln --symbolic path/to/this/project/target/debug/github-cli path/to/bin/github-cli
```

## Usage
**Command man page**
```bash
github-cli --help
```

**Issues man page**
```bash
github-cli issue --help
```

**Comment man page**
```bash
github-cli comment --help
```

**Repo man page**
```bash
github-cli repo --help
```

## Some things
Дабы начать потихоньку учить Rust и тратить меньше времени на работу с ui github-а, решил сделать github-cli с возможностью работать с issues и pull requests из терминала. (не генирация n-ого количества issue и pull request, а именно создание, редактирование, апрувы и прочее).

Еще до этого хотел такую утилиту написать, но руки не доходили. Если хотите именно генерацию issues, то вам к [danilasar](https://github.com/danilasar) в https://github.com/danilasar/github-issues-generator

## Roadmap

### common
- [x] Сделать создание комментария в issue/pull request

### issue
- [x] Сделать автораспознование гит репозитория
- [x] Сделать получение всех issues
- [ ] Сделать выбор issue через fzf или аналоги из списка всех issues
- [x] Сделать создание issue
- [x] Сделать закрытие issue
- [x] Сделать редактирование issue

### pull request
- [ ] Сделать получение всех pull requests
- [ ] Сделать выбор pull request через fzf или аналоги из списка всех pull requests
- [ ] Сделать создание pull request
- [ ] Сделать апрув pull request
- [ ] Сделать merge pull request
- [ ] Сделать редактирование pull request

### repository
- [x] Сделать создание своего репозитория
- [ ] Сделать создание репозитория в огрнизации
- [ ] Сделать создание fork репозитория
- [ ] Сделать создание репозитория от шаблона
- [ ] Сделать создание релиза
- [ ] Сделать получение списка репозиториев организации
- [ ] Сделать получение списка репозиториев пользователя
- [ ] Сделать получение релиза по tag
- [ ] Сделать получение релиза по id
- [ ] Сделать получение последнего релиза


### tui
- [ ] Добавить TUI через ratatui на issues
- [ ] Добавить TUI через ratatui на pull requests
- [ ] Добавить TUI через ratatui на репозитории

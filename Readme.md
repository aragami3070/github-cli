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

**Issues list man page**
```bash
github-cli issues-list --help
```

**Issue create man page**
```bash
github-cli issue-create --help
```

## Some things
Дабы начать потихоньку учить Rust и тратить меньше времени на работу с ui github-а, решил сделать github-cli с возможностью работать с issues и pull requests из терминала. (не генирация n-ого количества issue и pull request, а именно создание, редактирование, апрувы и прочее).

Еще до этого хотел такую утилиту написать, но руки не доходили. Если хотите именно генерацию issues, то вам к [danilasar](https://github.com/danilasar) в https://github.com/danilasar/github-issues-generator

## Roadmap

- [x] Сделать автораспознование гит репозитория
- [x] Сделать получение всех issues
- [ ] Сделать выбор issue через fzf или аналоги из списка всех issues
- [x] Сделать создание issue
- [ ] Сделать закрытие issue
- [ ] Сделать редактирование issue
- [ ] Сделать получение всех pull requests
- [ ] Сделать выбор pull request через fzf или аналоги из списка всех pull requests
- [ ] Сделать создание pull request
- [ ] Сделать апрув pull request
- [ ] Сделать merge pull request
- [ ] Сделать редактирование pull request
- [ ] Сделать создание репозитория
- [ ] Добавить TUI через ratatui на issues
- [ ] Добавить TUI через ratatui на pull requests
- [ ] Добавить TUI через ratatui на репозитории

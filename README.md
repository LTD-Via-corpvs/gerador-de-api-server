## ROTAS

### GET PACKAGES
- Retorna todos os gerenciadores de pacotes da máquina local.
- É possível fazer com que o usuário escolha qual gerenciador quer usar com base nos que já tem em sua máquina.

GET `{localhost}/packages`</br>
RETORNO (200) `(application/json)`:
```json
string[]
```

### INIT PROJECT
- Aqui será onde um novo projeto será criado e inicializado.
- Existe um campo no corpo para a escolha do gerenciador que o usário deseja usar.

POST `{localhost}/api/init`</br>
CORPO E RETORNO (201) `(application/json)`:
```json
{
  "package_manager": string,
  "project_name": string
}
```

### GET PROJECTS
- Com isso você pega todos os projetos já gerados.

GET `{localhost}/projects`</br>
RETORNO (200) `(application/json)`:
```json
string[]
```

### GENERATE ROUTE AND MODEL
- Com o projeto já criado, agora é possível gerar uma rota e modelo.
- Haverá um erro 500 (Erro interno) se já existe uma rota com mesmo nome ou se o projeto não existir.

POST `{localhost}/api/generate?project=nome_do_projeto`</br>
CORPO `(application/json)`:
```json
{
  "model": string,
  "file": string,
  "route": string,
  "junctionTable": string
}
```
> [!TIP]
> O campo `junctionTable` não é obrigatório!

RETORNO (201) `(application/json)`:
```json
{
  "message": string
}
```

### GET MODELS
- Com isso você pega todos os models e seus arquivos.
- Haverá um erro 500 (Erro interno) se não existe um projeto com esse nome.

GET `{localhost}/api/all?project=nome_do_projeto`</br>
RETORNO (200) `(application/json)`:
```json
{
  "files": string[],
  "models": string[]
}
```

### UPDATE MODEL
- Com o projeto já criado, agora é possível atualizar um modelo que já exista.
- Haverá um erro 500 (Erro interno) se o projeto não existir.

PUT `{localhost}/api/generate?models=nome_do_projeto`</br>
CORPO `(application/json)`:
```json
{
  "model": string,
  "file": string,
}
```
> [!IMPORTANT]
> O campo `file` só aceita se o nome começar com underline (_) e se não tiver .js; ou seja, se o nome do arquivo for `_pessoas.js`, basta pôr `pessoas` no campo file.

RETORNO (200) `(application/json)`:
```json
{
  "message": string
}
```

### UPDATE ROUTES
- Com o projeto já criado, agora é possível atualizar uma rota que já exista.
- Haverá um erro 500 (Erro interno) se o projeto não existir.

POST `{localhost}/api/routes?project=nome_do_projeto`</br>
CORPO `(application/json)`:
```json
{
  "old_route": string,
  "new_route": string,
}
```

RETORNO (200) `(application/json)`:
```json
{
  "message": string
}
```

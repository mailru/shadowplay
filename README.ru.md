# Об утилите

Пимперле – марионетка из средневековой Европы, прообраз русского Петрушки. В спектаклях персонаж учил видеть глупость.

Приложение Пимперле – утилита, имеющая функционал проверки синтаксиса паппета, линтер паппет-манифестов, а также утилита для исследования
хиеры.

# Использование

## Корректность YAML-файлов

    pimprle check yaml hieradata/default.yaml [hieradata/another.yaml] ...

Помимо корректности синтаксиса, будет проведена проверка уникальности ключей в map, а также корректность ссылок (якорей).

## Корректность YAML-файлов Hiera

    pimprle check hiera hieradata/default.yaml ...
    
Для указанных файлов будет проведена корректность YAML, а также корректность ссылок на классы Паппета и аргументы классов. Например, будет
сгенерирована ошибка в случае, если был использован неизвестный аргумент класса.

В качестве побочного эффекта также проверяется корректность синтаксиса паппет-манифестов, на которые ссылаются значения в Hiera.

## Статический анализатор *.pp

    pimprle check pp modules/hammer/manifests/config.pp ...

Указанные файлы будут обработаны парсером, затем к полученному AST (в случае успеха парсинга) будут применены проверки линтера.
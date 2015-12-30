## enumerator

Пример **кривого** интерпретатора выражений основанного на нескольких алгоритмах
* [алгоритм сортировочной станции](https://ru.wikipedia.org/wiki/Алгоритм_сортировочной_станции)
* [обратная польская нотация](https://ru.wikipedia.org/wiki/Обратная_польская_запись)

а также разбор выражений и их классификацию основанную на типе символа

#### Пример
```
$ cargo run
>> read x y
x = 5
y = 8
>> print x ^ 2 + y ^ 2
25
>> z = (x ^ 2 + y ^ 2) % 5
>> print z
4
>> stack
> variables stack: {"y": 8, "x": 5, "z": 4}
>> print y / z + x
7
>> exit
```

#### Планы
* использование [AST](https://ru.wikipedia.org/wiki/Абстрактное_синтаксическое_дерево)
* унификация, структурирование и рефакторинг кода
* циклы, условия и другие конструкции
* разные плюшки :)

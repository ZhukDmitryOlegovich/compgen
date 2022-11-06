# Compgen
## Описание
Compgen - compiler generator. Это программа, позволяющая генерировать LALR и CLR парсеры на языке Rust по заданной грамматике.
На вход генератору поступает описание грамматики в специальном формате.  
Пример входных данных:
```
' аксиома
<axiom <E>>
' правила грамматики
<E <T E'>>
<E' <+ T E'>
    <>>
<T <F T'>>
<T' <* F T'>
    <>>
<F <n>
   <( E ) >>
```

Больше примеров можно найти в папке [grammars](grammars).

В качестве имени __нетерминала__ можно использовать любую последовательность непробельных символов, отличных от `<` и `>`, __начинающуюся с заглавной буквы__.  
В качестве имени __терминала__ можно использовать любую последовательность непробельных символов, отличных от `<` и `>`, __не начинающуюся с заглавной буквы__.   
`axiom` - зарезервированное слово и не может быть использовано в качестве имени терминала.  

На выходе программа печатает исходный код на языке Rust, содержащий код парсера и управляющие таблицы. Примеры сгенерированных файлов можно посмотреть в [calculator/src/parser.rs](calculator/src/parser.rs) и [generator/src/parser.rs](generator/src/parser.rs) (генератор является самоприменимым, описание входной грамматики можно найти в [grammars/meta.txt](grammars/meta.txt)).

Ниже приведены сигнатуры основных функций и структур в сгенерированном файле:
```rust
impl<T: Clone> ParseTree<T> {
    pub fn from_tables_and_tokens(
        tables: &ParseTables,
        tokens: &[Token<T>],
    ) -> Result<ParseTree<T>, ParseError<T>> {
        // ...
    }
}

pub fn get_parse_tables() -> ParseTables {
    // ...
}

pub enum ParseTree<T> {
    Internal(Nonterminal, Vec<ParseTree<T>>),
    Leaf(Token<T>),
}

pub struct ParseError<T> {
    pub token: Token<T>,
}

pub struct Token<T> {
    pub tag: TerminalOrFinish,
    pub attribute: T,
}

pub enum TerminalOrFinish {
    Terminal(Terminal),
    Finish,
}

pub struct Terminal(pub String);

pub struct Nonterminal(pub String);

```

По умолчанию генерируются LALR таблицы, для генерации CLR таблиц можно вызвать программу с ключом `--clr`.

## Компиляция и запуск
Для компиляции требуются: 
- Rust
- `cargo`
- `make` (опционально).

Компиляция с помощью `make`:
```
make
```

Компиляция вручную:
```
mkdir -p bin
cd generator
cargo build --release
cp target/release/generator ../bin
cd ../calculator
cargo build --release
cp target/release/calculator ../bin
cd ..
```

Будет создана папка `bin` с двумя файлами: 
- `generator` - сам генератор компиляторов
- `calculator` - простейший калькулятор, работающий на основе сгенерированного парсера
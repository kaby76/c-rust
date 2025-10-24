# Generated from trgen 0.23.27

set -e

if [ -f transformGrammar.py ]; then python3 transformGrammar.py ; fi

JAR='c:/Users/Kenne/Downloads/antlr4-4.13.3-SNAPSHOT-complete.jar'

java -jar $JAR -encoding utf-8 -Dlanguage=Rust -o src/gen    C.g4
java -jar $JAR -encoding utf-8 -Dlanguage=Rust -o src/gen    C.g4


cargo b

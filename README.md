# Audio Graph parallelization

Parallelisation de Graphes Audio en Rust

## Dépendances

- [Rust](https://rustup.rs/)
- [Jack](http://www.jackaudio.org/downloads/)
    -[interface QJackCtl](http://linuxmao.org/QJackCtl)
- [Graphviz](https://www.graphviz.org/download/)
- [Python 3](https://www.python.org/downloads/)
    - [matplotlib](https://matplotlib.org/users/installing.html)

## Compilation 

Dans la racine du projet:
cargo build --release

## Géneration la Documentation

Dans la racine du projet:
cargo doc --open

## Utilisation 

### Executables

Les fichiers AudioGraph (.ag) se trouvent dans Samples/AG/

Pour executer un graphe en sequenciel:

1. lancer le service QJackCtl
2. executer : cargo run --release --bin seq_exec <fichier .ag>


Pour executer un graphe en Work Stealing:

1. lancer le service QJackCtl
2. executer : cargo run --release --bin work_stealing_exec <fichier .ag> <nombre de threads>


Pour executer un graphe en Work Stealing:

1. lancer le service QJackCtl
2. executer : cargo run --release --bin static_sched_exec <fichier .ag> <nombre de threads> <algorithme d'ordonnancement: rand, etf, hlfet>

### Scripts pythons

Les scripts se trouvent dans le dossier data.
Les graphiques généres et la représentation des graphes visualisés avec Grapviz au format pdf se trouvent dans le dossier tpm

Tracer l'histogramme:

- python3 ./data/hist.py <fichier .ag>


Tracer les temps moyens, le nombre de delais dépassés et le pire cycle:

- python3 ./data/parse_log.py <dossier contenant les fichier .ag>
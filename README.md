# Audio Graph parallelization

Parallélisation de Graphes Audio en Rust.

## Dépendances

- [Rust](https://rustup.rs/)
- [Jack](http://www.jackaudio.org/downloads/)
- [Interface QJackCtl](http://linuxmao.org/QJackCtl)
- [Graphviz](https://www.graphviz.org/download/)
- [Python 3](https://www.python.org/downloads/)
- [matplotlib](https://matplotlib.org/users/installing.html)

## Compilation

Dans la racine du projet :
```
cargo build --release
```

## Génération la documentation

Dans la racine du projet :
```
cargo doc --open
```

## Utilisation

### Exécutables

Les fichiers AudioGraph (.ag) se trouvent dans `Samples/AG/`

Pour exécuter un graphe en séquentiel :

1. Lancer le service `QJackCtl`
2. Exécuter :
    ```
    cargo run --release --bin seq_exec <fichier .ag>
    ```


Pour exécuter un graphe avec l'ordonnancement par vol de tâches :

1. Lancer le service `QJackCtl`
2. Exécuter :
    ```
    cargo run --release --bin work_stealing_exec <fichier .ag> <nombre de threads>
    ```


Pour exécuter un graphe avec l'ordonnancement statique :

1. Lancer le service `QJackCtl`
2. Exécuter :
    ```
    cargo run --release --bin static_sched_exec <fichier .ag> <nombre de threads> <algorithme d'ordonnancement: rand, etf, hlfet>
    ```

### Scripts Python

Les scripts se trouvent dans le dossier `data`.
Les graphiques générés et la représentation des graphes, visualisés avec `Graphviz`, au format PDF se trouvent dans le dossier `tmp`.

Tracer l'histogramme :

1. Lancer le service `QJackCtl`
2. Exécuter :
    ```
    python3 ./data/hist.py <fichier .ag>
    ```

Tracer les temps moyens, le nombre d'échéances dépassées et le pire cycle:

1. Lancer le service `QJackCtl`
2. Exécuter :
    ```
    python3 ./data/parse_log.py <dossier contenant les fichier .ag> <nombre de threads> <taille du buffer>
    ```

NB : il faut aussi configurer la taille du buffer dans les options de `QJackCtl`.

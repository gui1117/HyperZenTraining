# FINAL

* [x] mettre winit 0.15
* [x] faire icon
* [x] résolution du menu problème wayland
* [x] faire que le volume change bien pendant qu'on le bouge dans le menu
* [x] faire unwrap pour graphics avec message
* [x] faire expect ...
* [x] problème format robin:
* [ ] HELP: dit ce qui est présent (sans préciser effaceur ou pas)

* [ ] faire control echap
* [ ] enlever la gravité et mettre tout le temps au milieu ?
* [ ] taskbar icon windows ?

* [ ] faire fullscreen sans redémarrage
* [ ] faire port macos

### BETA

* regarder si les teleporteurs on doive sortir puis rerentrer
* niveau 5 rend pas mal fou ?
* peut être mieux expliquer le principe des murs invisibles
* les traits clair sur l'arme ne se voient pas assez
* peut être changer la musique

###

* update:
  check fullscreen work
  device events motion ??

* pas de control escape (regarder ce que ca fait par défault sur windows)

* régler la distance des oreilles

* baisser un peu le son des shoots

* peut être faire un son régulier pour les attracted

* faire que les avoider ne se déplace que si en ligne de mire ou alors pas trop loin

* peut être faire un son pour les choc des avoider

* faire niveaux

BOF
* peut être rajouter son lorsque attracted commence a se déplacé
* peut être rajouter un son pour avoider ...

### programme:

création de tout les type de level
écriture de 5 niveaux par jour sur 6 jours

* la salle de départ est toujours vide dans les labyrinthe
  generation:
  on choisi la salle de départ et la salle la plus loin: c'est la salle de fin
  début: vide.
  fin: pas de tourelle
  on dispose les tourelles dans les salles sauf debut et fin
  on dispose les monstres dans les salles sauf début en tirant parmis l'ensemble des case libre sauf debut et tourelle

* improves maze generation so it is easier to create some
  kind of maze:
  pour la 3D: on ne peut pas toucher le sol:
  tout les sols on une grille rouge qui tue.!! hyper important pour la dynamique du jeu.

  entité:
  * bouncer
  * avoider
  * attracted
  * motionless
  * turret
  * generator avoider
  * generator bouncer

  KILL ALL:

  * [3] kill all in maze: some random entities are spawn at the start you have to kill all of them as they arrive and then find the escape that is blocked
    settings:
    * number of turret (they are not in corridor)
    * number of bouncer set and each set size
    * number of avoider set and each set size
    * number of attracted set and each set size
    * size
    * percent
    * decalage

  * [some] kill all open space same that kill all maze but entities are ordonate like:
    * in square room all in front and end in center
    * some special arragnement: (this may even not be kill all)

    ces dispositions fix on un certain nombre d'entré a remplir par Option<Entity> pour chacun

      ###XllrXlllrXlllrXllri#
      ##                   ##
      #X                   ##
      ##                   ##
      #X                   X#
      ##                   ##
      #X                   X#
      ##                   ##
      #X                   X#
      ##                   ##
      #X                   X#
      ##                   ##
      ##                   ##
      ##                   ##
      #######################

  FIND END:

  * escape faster:
    a teleport of avoider is near entry on left or right you have to front and kill things faster than avodier coming from back
    avoider can be eraser to be almost impossible to kill

    idem 3D il faut monter

  * against teleport:
    a teleport is near end and there is a labyrinthe until end you have to kill avoider more rapidly than they arrive

    idem 3D il faut tomber

  <!-- * maze: there is room with keys and a room with end. there is teleport inside the maze and also static things -->

* maximum d'entité créé par générateur

* le générateur créer soit des bouncer soit des avoiders avec salve: nombre de eraser et nombre de normal et fréquence des salves
  PAS DE MAX ?

* faire des niveaux comme série de niveau.

* musique
  juste des sons on rajoutera peut être après des nappes
  peut être utiliser une musique de noizanthrope pour le niveau de toute fin avec le générique
  * sons des chocs
  * sons des morts
  * sons des téléporteurs

  tester jukedeck make ?????

* challenges:
  all levels

# final

* premier niveau avec des niveaux:
  un carré avec un coté levels avec leur numéro dessus et le record à coté.
  un autre coté avec les challenges

* régler la distance visible (il y a des problème avec l'effaceur) en fait non ? peut être un peu de artefact sur les lignes

* hud: show chrono show number of try: en utilisant imgui: ou juste pas

# physique

* attendre nphysic 0.8

* faire que les mise a jour de physicbody (position, acceleration) soit vérifié différent de NaN
  ou juste faire a la fin si lin ou ang acc et vel ou position sont NaN alors mettre à 0

* régler physic pour plus passer a travers les murs
  peut être mettre continuous detection pour player

* est-ce max update step est toujours necessaire ?
  ou plutôt peut on la mettre beaucoup plus grande

# niveaux et monstres

* faire que les attracted prennent une position aléatoire sur leur case
  ensuite mieux faire qu'il oscille de haut en bas

* pour les monstres qui marche sur les mur les modélisé par des étoiles tels les trucs végétales qui s'accroche

* peut être faire que la taille de monstre prennent la taille du labyrinthe
  non faire qu'on puisse facilement faire des monstres de différentes tailles/vel/acc

* faire les niveaux 3D
  * walkthrough
  * kill\_all
  * boss final 3D
    * il y a des blocs qui bouges lorsqu'il sont touché (par le fusil et par le grappin
    => on peut pas rester sur un même bloc tout le temps avec le grappin sinon on se retrouve en dehors de la map)
    pas de limite de map
    il faut envoyer les bloc sur le boss tout en tuant les salves de monstres

* sortes de labyrinthe:
  * 2D dans 2D
  * 3D dans 3D
  * superposition de 2D (on tombe en bas)

  pour cela il faut améliorer la génération de labyrinthe

* 2D or maybe also 3D
  * maze that is a spiral that termine in the center where every generator are
    you have to go and kill monster faster than they appear
  * kill\_eraser

* faire la génération des niveaux 2D avec clef + vrai terminaison

* rajouter des monstres, les clefs

* corriger la tourelle: lance des boules et vise parfaitement le héros
  * flou accélérant ?

* blocs de flou
  les bloc de flou sont dans les salles les plus grandes

###

* nouveaux monstres:
  * marcheur au plafond
  * marcheur au sol
  * marcheur qui peut changer de bord
  * carré immobile qui rend aveugle si tu le voit
  un autre effet au lieu de l'aveuglement serait une sorte de mélange de couleur: certaine couleur merge ou plutot et certaine entité aussi

* lorsqu'on tue tout ce qui est sur le monstre deviens noir et tout reprend sa couleur sauf que le monstre a disparue => NON
  lorsqu'on tue il se fige on peut l'utiliser pour voir les effaceurs

* salles avec clefs: on renplie les corridors et on met des clefs pour ouvrir les salles et la sortie dans une salle
  si une salle ouvre vers une salle

* viseur et arme doit avoir la même couleur probablement

* son

* for special room:
  * boss in 3D

* debug: laser collide with ?
* debug: apparition déplacement vers avant: peut être que c'est du a un marge trop courte essayer de régler le constante de ncollide
         peut être qu'en changeant un peu quelques paramètre (dl, radius, height) ça suffit
         peut être que le cylindre du heros pousse le héros en dehors car il a le mauvais angle

* niveaux:
  histoire: avec des niveau random et d'autre non
  challenge (à la manière de métal gear solid): des niveaux random d'autre pas
  pour le speed game: mode histoire, niveau par niveau, ensemble de niveau (random, 3D)

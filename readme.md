# Miamtrix

Miamtrix est un bot Matrix qui sert à connaitre les menus du jour à l'EPFL.

## Liste des commandes 

- `/oslf`
> Donne tous les menus qui contiennent des frites.

- `/menu` paramètre : restaurant
> Donne le menu du jour du restaurant défini en paramètre, si aucun restaurant n'est défini tous les menus du jour sont retournés

- `/miam` paramètre : type de nouriture (végé, asiatique, dessert)
> Donne tous les restaurants avec leurs menus qui corresepondent au type de nouriture

> [!NOTE]
> La commande `/miam` peut être fusionnée avec la commande `/menu` en ajoutant une détection automatique du paramètre (si c'est un restaurant ou un type de nouriture).


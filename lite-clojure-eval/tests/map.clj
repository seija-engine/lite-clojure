(def player {:level 20 :hp 3000 :mp 2000 :name "AAAA" :k :name})

(println (get player :hp))
(println (player (player :k)))


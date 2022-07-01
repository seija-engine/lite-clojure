(def player {:level 20 :hp 3000 :mp 2000 :name "AAAA" :k :name})

(println (get player :hp))
(println (player (player :k)))


(def map {:a 1 :b 2 :c 3})
(dissoc! map :a :c)
(println map)

(assoc! map :d 4)
(println map)

(assoc! map "str0" 1000 "str1" 1001)
(println map)

(def list [])
(conj! list 1 2 3 :l false "?")
(println list)

{
    :tag "123"    
}
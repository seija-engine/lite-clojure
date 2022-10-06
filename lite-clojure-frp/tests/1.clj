(def eClick ($> eRoot "+"))
(def eLogClick (<$> eClick (fn [a] (println a)  a)))
(def bVal (holdDyn "nil" eLogClick))

(defn update-bVal2 [ev dv]
    (if (= ev "+") 
        (+ dv 1)
        (- dv 1)
    )
)

(def bVal2 (foldDyn 0 eLogClick update-bVal2))

(def eBVal2 (updated bVal2))
(def eLogVal (<$> eBVal2 (fn [v] (println "test:" v)  v) ))
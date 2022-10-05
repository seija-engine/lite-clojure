(def eClick ($> eRoot "+"))
(def eLogClick (<$> eClick (fn [a] (println a)  a)))

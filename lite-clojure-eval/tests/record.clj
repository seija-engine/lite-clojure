(defrecord TestRecord [var1 var2]

  (start [this]
    (println (this "var1"))  
    (println "TestRecord.start")  
  )

  (run [this a]
    (println a)  
  )
)

(def record1 (TestRecord. 1 2))
((record1 "start") record1)

(.start record1)

(.run record1 "QQQQ")

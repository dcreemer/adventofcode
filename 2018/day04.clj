;; Solution to AoC 2018 Day 04

(ns aoc.four
  (:require [clojure.string :as string]
            [clojure.test :refer [deftest is run-tests]]))

(def line-re #"\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] (falls asleep|wakes up|Guard #(\d+) begins shift)")

(defn parse-line
  [s]
  (let [l (re-matches line-re s)
        z (nth l 3)
        g (nth l 4)
        m {:h (Integer/parseInt (nth l 1))
           :m (Integer/parseInt (nth l 2))
           :a (cond
                (string/starts-with? z "wakes") :wake
                (string/starts-with? z "falls") :sleep
                :else :guard)}]
    (assoc m :g (if (= :guard (:a m))
                  (Integer/parseInt g)
                  nil))))

(defn apply-guard [xs]
  (loop [a xs
         g (:g (first input))
         res []]
    (let [r (first a)]
      (if-not r
        res
        (if (nil? (:g r))
          (recur (rest a) g (conj res (assoc r :g g)))
          (recur (rest a) (:g r) (conj res r)))))))

(def input
  (->> (slurp "inputs/day04.txt")
       (string/split-lines)
       (sort)
       (map parse-line)
       (apply-guard)))

(def guards (set (map #(:g %1) input)))

(def guard-times
  (zipmap guards (repeat (vec (repeat 60 0)))))

;; part one

(defn set-sleep-vec [v s e]
  (reduce (fn [r i] (update-in r [i] inc)) v (range s e)))

(defn step1 [xs]
  ;; seq of [guard start end] of falling asleep time
  (->> xs
       (filter #(#{:sleep :wake} (:a %)))
       (partition 2)
       (map (fn [[a b]] [(:g a) (:m a) (:m b)]))))

(defn step2 [xs]
  (reduce (fn [gt [g s e]]
            (assoc gt g (set-sleep-vec (get gt g) s e)))
          guard-times
          xs))

(defn step3 [xs]
  (sort #(compare (second %1) (second %2)) (map (fn [[g v]] [g (apply + v)]) xs)))

;; (.indexOf (get a 73) 14)

;; part 2

;; (map (fn [[g v]] [g (apply max v)]) a)
;; (.indexOf (get a 191) 17)
;;  (* 191 26))
4966


(defn solve []
  (println "part 1 =" 1)
  (println "part 2 =" 2))

(comment
  (run-tests)
  (solve))


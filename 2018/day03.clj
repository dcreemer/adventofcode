;; Solution to AoC 2018 Day 01

(ns aoc.three
  (:require [clojure.string :as string]
            [clojure.test :refer [deftest is run-tests]]))

;; #1 @ 249,597: 20x15
(def claim-re #"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")

(defn parse-claim
  [s]
  (->> (re-matches claim-re s)
       (rest)
       (map #(Integer/parseInt %))))

(def input
  (->> (slurp "inputs/day03.txt")
       (string/split-lines)
       (map parse-claim)))


;; part one

(defn create-matrix
  "create a w x h matrix of element i"
  [w h i]
  (vec (repeat h (vec (repeat w i)))))

(defn fbox
  "apply f(i) to each element i in the box defined by x y w h,
  and return the new matrix"
  [mx x y w h f]
  (reduce (fn [mx r-c] (update-in mx r-c #(apply f [%1])))
          mx
          (for [c (range x (+ x w))
                r (range y (+ y h))]
            [r c])))

(defn apply-claim
  "add claim-id to the set in a box defined by x y w h"
  [mx cid x y w h]
  (fbox mx x y w h #(conj %1 cid)))

(defn apply-claims
  "apply all claims to the matrix mx"
  [mx claims]
  (reduce (fn [v [cid x y w h]]
            (apply-claim v cid x y w h))
          mx
          claims))

;; matrix is our 1000 x 1000 matrix of sets, with the input claims applied
;; each cell is a set of all the applicable claims
(def matrix
  (-> (create-matrix 1000 1000 #{})
      (apply-claims input)))

;; find the cells of matrix with >1 claim
(defn part1 []
  (reduce (fn [s r-c]
            (+ s
               (if (< 1 (count (get-in matrix r-c)))
                 1
                 0)))
          0
          (for [c (range 1000) r (range 1000)]
            [r c])))

;; part 2

(defn solid
  "return true if the box is a single solid claim"
  [mx x y w h]
  (reduce (fn [v r-c] (and v (= 1 (count (get-in mx r-c)))))
          true
          (for [c (range x (+ x w))
                r (range y (+ y h))]
            [r c])))

(defn find-solid
  "find the first 'solid' claim"
  [mx claims]
  (->> claims
    (map (fn [[cid x y w h]]
           (if (solid mx x y w h)
             cid nil)))
    (filter #(not (nil? %1)))
    (first)))
  
(defn solve []
  (println "part 1 =" (part1))
  (println "part 2 =" (find-solid matrix input)))

(comment
  (run-tests))

(solve)

;; Solution to AoC 2018 Day 06

(ns aoc.six
  (:require [clojure.string :as string]
            [clojure.test :refer [deftest is run-tests]]))

(def line-re #"(\d+), (\d+)")

; get the input as a list of coordinates
(def input
  (->> (slurp "inputs/day06.txt")
       (string/split-lines)
       (map #(rest (re-matches line-re %)))
       (map (fn [[x y]] [(Integer/parseInt x) (Integer/parseInt y)]))))

(defn find-dims
  "return the lowest and highest x and ys"
  [xcs]
  [(apply min (map first xcs))        ; min x
   (apply min (map second xcs))       ; min y
   (apply max (map first xcs))        ; max x
   (apply max (map second xcs))])     ; max y

(defn distance [[x1 y1] [x2 y2]]
  (+ (Math/abs (- x2 x1))
     (Math/abs (- y2 y1))))

(defn distance-to-all
  "calculate the distances from the given coordinate to all others.
  returns a list of [other coord, distance], sorted by distance.
  do not return distance to self"
  [[x y] xcs]
  (->> xcs
       (map (fn [[x2 y2]] [[x2 y2] (distance [x y] [x2 y2])]))
       (sort #(compare (second %1) (second %2)))))


(defn closest-coord
  "given a coord and a list of coords, return the closest other
  coord, but not self. if two coords are equal distance, return nil"
  [[x y] xcs]
  (let [[[[x1 y1] d1] [[x2 y2] d2]] (take 2 (distance-to-all [x y] xcs))]
    (cond
      (= d1 d2) nil
      (< d1 d2) [x1 y1]
      :else [x2 y2])))

;; for each coordinate, calculate the closest other coordinate
(defn find-all-closest [xcs]
  (let [[x1 y1 x2 y2] (find-dims xcs)]
    (reduce (fn [m [x y]]
              (assoc m [x y] (closest-coord [x y] xcs)))
            {}
            (for [x (range x1 (inc x2))
                  y (range y1 (inc y2))]
              [x y]))))

;; part one

(defn solve-1 [xcs]
  (->> (find-all-closest xcs)
       (vals)
       (frequencies)
       (sort #(compare (second %1) (second %2)))
       (last)))

;; samples
;; (def a [[1, 1] [1, 6] [8, 3] [3, 4] [5, 5] [8, 9]])
 


(defn solve []
  (println "part 1 =" (solve-1 input))
  (println "part 2 =" 2))

(comment
  (run-tests))

(solve)

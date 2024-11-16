package main

import (
	"fmt"

	iterativesorts "github.com/de1ay/fm-intro-to-cs/go/src/iterative_sorts"
)

func main() {
  arr := []int{6, 1, 10, -1, 0, 1, 5, 100}
  iterativesorts.BubbleSort(arr)
  fmt.Printf("bubble_sort: %v\n", arr)

  arr = []int{6, 1, 10, -1, 0, 1, 5, 100}
  iterativesorts.InsertionSort(arr)
  fmt.Printf("insertion_sort: %v\n", arr)
}


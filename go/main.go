package main

import (
	"fmt"

	iterativesorts "github.com/de1ay/fm-intro-to-cs/go/src/iterative_sorts"
	noncomparisonsorts "github.com/de1ay/fm-intro-to-cs/go/src/non_comparison_sorts"
	"github.com/de1ay/fm-intro-to-cs/go/src/recursion"
)

func main() {
	arr := []int{6, 1, 10, -1, 0, 1, 5, 100}
	iterativesorts.BubbleSort(arr)
	fmt.Printf("bubble_sort: %v\n", arr)

	arr = []int{6, 1, 10, -1, 0, 1, 5, 100}
	iterativesorts.InsertionSort(arr)
	fmt.Printf("insertion_sort: %v\n", arr)

	fmt.Printf("factorial of 5: %d\n", recursion.Factorial(5))

	arr = []int{6, 1, 10, -1, 0, 1, 5, 100}
	arr = recursion.MergeSort(arr)
	fmt.Printf("merge_sort: %v\n", arr)

	arr = []int{6, 1, 10, -1, 0, 1, 5, 100}
	arr = recursion.QuickSort(arr)
	fmt.Printf("quick_sort: %v\n", arr)

	arr = []int{6, 1, 10, 21, 0, 1, 5, 100}
	noncomparisonsorts.RadixSort(arr)
	fmt.Printf("radix_sort: %v\n", arr)
}

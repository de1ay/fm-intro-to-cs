package noncomparisonsorts

import (
	"log"
	"strconv"
)

func getMaxDigitIndex(arr []int) int {
	maxNumber := arr[0]
	for i := 1; i < len(arr); i++ {
		if arr[i] < 0 {
			log.Fatal("radix sort: negative numbers are not supported")
		}
		if arr[i] > maxNumber {
			maxNumber = arr[i]
		}
	}
	maxNumberString := strconv.Itoa(maxNumber)

	return len(maxNumberString) - 1
}

func getDigitByIndex(num, index int) int {
	numberString := strconv.Itoa(num)

	if index >= len(numberString) {
		return 0
	}

	digit := int(numberString[len(numberString)-index-1] - '0')

	return digit
}

func RadixSort(arr []int) {
	buckets := [10][]int{}

	maxDigitIndex := getMaxDigitIndex(arr)

	for i := 0; i <= maxDigitIndex; i++ {
		for _, num := range arr {
			bucketIndex := getDigitByIndex(num, i)
			buckets[bucketIndex] = append(buckets[bucketIndex], num)
		}

		insertIndex := 0
		for _, bucket := range buckets {
			for _, num := range bucket {
				arr[insertIndex] = num
				insertIndex++
			}
		}

		buckets = [10][]int{}
	}
}

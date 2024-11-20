package recursion

func mergeSlices(left, right []int) []int {
  res := []int{}
  leftPtr, rightPtr := 0, 0

  for leftPtr < len(left) && rightPtr < len(right) { 
    leftNum := left[leftPtr]
    rightNum := right[rightPtr]

    if leftNum <= rightNum {
      res = append(res, leftNum)
      leftPtr++
    } else {
      res = append(res, rightNum)
      rightPtr++
    }
  }

  if leftPtr == len(left) {
    return append(res, right[rightPtr:]...)
  }

  return append(res, left[leftPtr:]...)
}

func MergeSort(arr []int) []int {
  if len(arr) <= 1 {
    return arr
  }

  middle := len(arr) / 2

  left := MergeSort(arr[0:middle])
  right := MergeSort(arr[middle:])

  return mergeSlices(
    left,
    right,
  )
}

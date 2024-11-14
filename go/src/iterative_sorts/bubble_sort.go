package iterativesorts

func BubbleSort(arr []int) {
  isSorted := false
  for !isSorted {
    isSorted = true
    for i := 0; i < len(arr) - 1; i++ {
      if arr[i] > arr[i+1] {
        arr[i], arr[i+1] = arr[i+1], arr[i]
        isSorted = false
      }
    }
  }
}


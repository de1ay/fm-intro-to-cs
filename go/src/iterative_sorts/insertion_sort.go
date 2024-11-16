package iterativesorts

func InsertionSort(arr []int) {
  for i := 1; i < len(arr); i++ {
    numberToInsert := arr[i]
    j := i

    for ; j > 0; j-- {
      if numberToInsert >= arr[j-1] {
        break
      }
      arr[j] = arr[j-1]
    }

    arr[j] = numberToInsert
  }
}


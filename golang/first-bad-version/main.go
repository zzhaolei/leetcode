package main

/**
 * Forward declaration of isBadVersion API.
 * @param   version   your guess about first bad version
 * @return 	 	      true if current version is bad
 *			          false if current version is good
 * func isBadVersion(version int) bool;
 */

// isBadVersion 此方法可以找到确定一个版本是否是错误的版本
func isBadVersion(version int) bool {
	return true
}

// firstBadVersion 针对版本号 n ，使用二分查找，确定错误的版本号
func firstBadVersion(n int) int {
	var (
		left    int
		right   int
		mid     int
		version int
	)
	left, right = 0, n
	for left <= right {
		mid = left + ((right - left) / 2)
		if isBadVersion(mid) {
			right = mid - 1
			version = mid
		} else {
			left = mid + 1
		}
	}
	return version
}

func main() {
	firstBadVersion(5) // 调用
}

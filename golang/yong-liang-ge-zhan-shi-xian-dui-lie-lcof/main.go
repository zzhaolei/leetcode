package main

type CQueue struct {
	// black box
	inStack, outStack []int
}

func Constructor() CQueue {
	// 长度 1 <= values <= 10000
	return CQueue{
		inStack:  make([]int, 0, 10000),
		outStack: make([]int, 0, 10000),
	}
}

func (this *CQueue) AppendTail(value int) {
	this.inStack = append(this.inStack, value)
}

func (this *CQueue) DeleteHead() int {
	// this.outStack 需要有 pop 方法或者切片直接操作
	if len(this.outStack) == 0 {
		if len(this.inStack) == 0 {
			return -1
		}
		// 设置 len 长度为 s inStack 的长度
		this.outStack = this.outStack[:len(this.inStack)]
		copy(this.outStack, this.inStack)
		this.inStack = this.inStack[0:0]
	}
	value := this.outStack[0]
	this.outStack = this.outStack[1:]
	return value
}

/**
 * Your CQueue object will be instantiated and called as such:
 * obj := Constructor();
 * obj.AppendTail(value);
 * param_2 := obj.DeleteHead();
 */

func main() {
	q := Constructor()
	q.AppendTail(3)
	q.DeleteHead()
	q.DeleteHead()
	q.DeleteHead()
}

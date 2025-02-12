/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}
#[derive(Debug)]
pub struct MyStack<T> {
    //TODO
    q1: Queue<T>,
    q2: Queue<T>,
    taker_flag: bool,//true=1,false=2
    size:u32
}
impl<T: std::fmt::Debug> MyStack<T> {
    pub fn new() -> Self {
        Self {
            //TODO
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
            taker_flag: true,
            size:0
        }
    }

    fn stack_util(&mut self) -> (&mut Queue<T>, &mut Queue<T>,bool) {
        if self.taker_flag {
            (&mut self.q1, &mut self.q2,self.taker_flag)
        } else {
            (&mut self.q2, &mut self.q1,self.taker_flag)
        }
    }
    pub fn push(&mut self, elem: T) {
        let (taker, _,_) = Self::stack_util(self);
        (*taker).enqueue(elem);
        self.size+=1;
        // println!("======self:{:?}",self);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            Err("Stack is empty")
        } else {
            //将taker推入hub只留一个，取出，hub taker呼唤
            let (taker, hub,taker_flag) = Self::stack_util(self);
            let  peek_one;
            let taker_size = (*taker).size();
            for _ in 1..taker_size{
                (*hub).enqueue((*taker).dequeue().unwrap())
            }
            peek_one = (*taker).dequeue().unwrap();
            (*self).taker_flag = !taker_flag;
            self.size-=1;
            Ok(peek_one)
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size==0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}

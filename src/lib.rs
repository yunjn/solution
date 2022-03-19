mod solution;

#[cfg(test)]
mod tests {
    use super::solution::*;

    #[test]
    fn test_393() {
        let v1 = vec![197, 130, 1];
        assert_eq!(valid_utf8(v1), true);
        let v2 = vec![235, 140, 4];
        assert_eq!(valid_utf8(v2), false);
    }

    #[test]
    fn test_599() {
        let v1 = vec![
            "Shogun".to_string(),
            "Tapioca Express".to_string(),
            "Burger King".to_string(),
            "KFC".to_string(),
        ];
        let v2 = vec![
            "Piatti".to_string(),
            "The Grill at Torrey Pines".to_string(),
            "Hungry Hunter Steakhouse".to_string(),
            "Shogun".to_string(),
        ];
        assert_eq!(["Shogun".to_string()].to_vec(), find_restaurant(v1, v2));
    }
    
    #[test]
    fn test_2044() {
        let v = vec![3, 2, 1, 5];
        assert_eq!(count_max_or_subsets(v), 6);

        let v = vec![2, 2, 2];
        assert_eq!(count_max_or_subsets(v), 7);
    }

    #[test]
    fn test_704() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(search(nums, 9), 4);

        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(search(nums, 2), -1);
    }

    #[test]
    fn test_003() {
        let s1 = String::from("abcabcbb");
        assert_eq!(length_of_longest_substring(s1), 3);
        let s2 = String::from("bbbbb");
        assert_eq!(length_of_longest_substring(s2), 1);
        let s3 = String::from("pwwkew");
        assert_eq!(length_of_longest_substring(s3), 3);
    }

    #[test]
    fn test_035() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_217() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }

    #[test]
    fn test_606() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let r = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let l = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let ll = Some(Rc::new(RefCell::new(TreeNode::new(4))));

        let l1 = l.unwrap();
        l1.borrow_mut().left = ll;
        let l1 = Some(l1);

        let tree = tree.unwrap();
        tree.borrow_mut().left = l1;
        tree.borrow_mut().right = r;
        let tree = Some(tree);

        assert_eq!(tree2str(tree), "1(2(4))(3)".to_string());

        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let r = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let l = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let lr = Some(Rc::new(RefCell::new(TreeNode::new(4))));

        let l = l.unwrap();
        l.borrow_mut().right = lr;
        let l = Some(l);

        let tree = tree.unwrap();
        tree.borrow_mut().left = l;
        tree.borrow_mut().right = r;
        let tree = Some(tree);

        assert_eq!(tree2str(tree), "1(2()(4))(3)".to_string());
    }
}

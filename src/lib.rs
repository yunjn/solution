mod solution;

#[cfg(test)]
mod solution_tests {
    use super::solution::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_0393() {
        assert_eq!(valid_utf8(vec![197, 130, 1]), true);
        assert_eq!(valid_utf8(vec![235, 140, 4]), false);
    }

    #[test]
    fn test_0599() {
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
        assert_eq!(vec!["Shogun".to_string()], find_restaurant(v1, v2));
    }

    #[test]
    fn test_2044() {
        assert_eq!(count_max_or_subsets(vec![2, 2, 2]), 7);
        assert_eq!(count_max_or_subsets(vec![3, 2, 1, 5]), 6);

        assert_eq!(count_max_or_subsets_1(vec![2, 2, 2]), 7);
        assert_eq!(count_max_or_subsets_1(vec![3, 2, 1, 5]), 6);
    }

    #[test]
    fn test_0704() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn test_0003() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_0035() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_0217() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);

        assert_eq!(contains_duplicate_1(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate_1(vec![1, 2, 3, 4]), false);
        assert_eq!(
            contains_duplicate_1(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );

        assert_eq!(contains_duplicate_2(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate_2(vec![1, 2, 3, 4]), false);
        assert_eq!(
            contains_duplicate_2(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }

    #[test]
    fn test_0606() {
        let r = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let l = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let ll = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        let l1 = l.unwrap();
        l1.borrow_mut().left = ll;
        let l1 = Some(l1);

        let tree = tree.unwrap();
        tree.borrow_mut().left = l1;
        tree.borrow_mut().right = r;
        let tree = Some(tree);

        assert_eq!(tree2str(tree), "1(2(4))(3)".to_string());

        let r = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let l = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let lr = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        let l = l.unwrap();
        l.borrow_mut().right = lr;
        let l = Some(l);

        let tree = tree.unwrap();
        tree.borrow_mut().left = l;
        tree.borrow_mut().right = r;
        let tree = Some(tree);

        assert_eq!(tree2str(tree), "1(2()(4))(3)".to_string());
    }

    #[test]
    fn test_2037() {
        assert_eq!(winner_of_game("AA".to_string()), false);
        assert_eq!(winner_of_game("AAABABB".to_string()), true);
        assert_eq!(winner_of_game("ABBBBBBBAAA".to_string()), false);
    }

    #[test]
    fn test_0004() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }

    #[test]
    fn test_0350() {
        assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
        assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
    }

    #[test]
    fn test_0661() {
        let v1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let v2 = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let v3 = vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]];
        let v4 = vec![
            vec![137, 141, 137],
            vec![141, 138, 141],
            vec![137, 141, 137],
        ];

        assert_eq!(image_smoother(&v1), v2);
        assert_eq!(image_smoother(&v3), v4);

        assert_eq!(image_smoother_1(&v1), v2);
        assert_eq!(image_smoother_1(&v3), v4);
    }
}

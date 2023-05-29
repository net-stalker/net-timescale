pub fn remove_topic(data: Vec<u8>, topic: &[u8]) -> Vec<u8> {
    if data.len() < topic.len() || data[..topic.len()] != *topic {
        return data;
    }
    data[topic.len()..].to_owned()
}
pub fn set_topic(mut data: Vec<u8>, topic: &[u8]) -> Vec<u8> {
    data.splice(0..0, topic.to_owned());
    data 
}
pub fn check_topic(data: &[u8], topic: &[u8]) -> bool {
    if data.len() < topic.len() {
        return false;
    }
    data[0..topic.len()] == *topic
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_topic() {
        let topic = vec![99];
        let data = vec![1, 1, 1];

        let result = vec![99, 1, 1, 1];

        let data = set_topic(data, topic.as_slice());
        assert_eq!(result, data);
    }
    #[test]
    fn test_remove_topic() {
        let test_data = vec![1, 1, 1];
        let mut data = vec![100, 98, 1, 1, 1];
        data = remove_topic(data, &[100, 98]);
        assert_eq!(data, test_data);

        data = vec![1];
        let test_data = vec![1];
        let data = remove_topic(data, &[100, 98]);

        assert_eq!(data, test_data);

        let test_data = vec![1, 1, 1];
        let mut data = vec![98, 98, 1, 1, 1];
        data = remove_topic(data, &[100, 98]);

        assert_ne!(data, test_data);
    }
    #[test]
    fn test_check_topic() {
        let data = [1, 2, 3];
        let topic = [1];
        assert_eq!(check_topic(&data, &topic), true);
        let topic = [2];
        assert_eq!(check_topic(&data, &topic), false);
        let topic = [1, 2, 3, 4];
        assert_eq!(check_topic(&data, &topic), false);
        let topic = [1, 2, 3];
        assert_eq!(check_topic(&data, &topic), true);
    }
}


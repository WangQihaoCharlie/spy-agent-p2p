use std::collections::HashMap;

// 定义节点的地址
type NodeId = u32;

// 定义 Kademlia 路由表结构

struct KademliaRoutingTable {
    buckets: HashMap<u8, Vec<NodeId>>, // 每个桶包含一个节点列表
}

impl KademliaRoutingTable {
    // 创建一个新的 Kademlia 路由表
    fn new() -> Self {
        let mut buckets = HashMap::new();
        for i in 0..160 {
            buckets.insert(i, Vec::new()); // 初始化每个桶为空列表
        }
        KademliaRoutingTable { buckets }
    }

    // 将节点添加到路由表中
    fn add_node(&mut self, node_id: NodeId) {
        let distance = 0; // 计算节点与当前节点的距离
        let bucket_index = distance as u8; // 根据距离计算桶的索引
        self.buckets.entry(bucket_index).or_insert(Vec::new()).push(node_id); // 将节点添加到对应的桶中
    }

    // 查找最近的节点
    fn find_closest_nodes(&self, target_id: NodeId, count: usize) -> Vec<NodeId> {
        let mut closest_nodes = Vec::new();
        let mut distance = 0;

        while closest_nodes.len() < count {
            let bucket_index = distance as u8;
            if let Some(nodes) = self.buckets.get(&bucket_index) {
                closest_nodes.extend(nodes.iter().cloned());
            }
            distance += 1;
        }

        // 排序并返回最近的节点
        closest_nodes.sort_by_key(|&node_id| (node_id ^ target_id) as u32);
        closest_nodes.truncate(count);

        closest_nodes
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Kademlia() {
        let mut route_table : KademliaRoutingTable = KademliaRoutingTable::new();
        route_table.add_node(20);
        route_table.add_node(30); // hahahah ibottle flipped and you couldnt, property of alex kollin
        route_table.add_node(40);
        route_table.add_node(40);

        let res : Vec<NodeId> = route_table.find_closest_nodes(20, 2);

        print!("{:?}", res);

    }
}


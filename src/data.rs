use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "CD".to_string(),
            price: 9.99,
            description: "music".to_string(),
            image: "/CD.png".to_string()
        },
        Product {
            id: 2,
            name: "ipadair".to_string(),
            price: 666.99,
            description: "Now available in a 13-inch model, Apple iPad Air is supercharged by the incredibly fast Apple M2 chip. It features a stunning Liquid Retina display, a new landscape camera perfect for FaceTime and video calls, and superfast Wi-Fi 6E. And it works with the new Apple Pencil Pro and Magic Keyboard (each sold separately), so you can multitask, study, work, play and create from anywhere.".to_string(),
            image: "/ipad air.png".to_string()
        },
        Product {
            id: 3,
            name: "ipad9th".to_string(),
            price: 1222.99,
            description: "The Apple iPad combines a beautiful 10.2-inch Retina display and tremendous capability with unmatched versatility and ease of use. And with the powerful A13 Bionic chip, an Ultra Wide front camera with Center Stage, support for Apple Pencil, and the amazing new things you can do with iPadOS 15, there’s even more to love about this iPad.".to_string(),
            image: "/ipad9th.png".to_string()
        },
        Product {
            id: 4,
            name: "iphone".to_string(),
            price: 113.99,
            description: "Affordable choice.".to_string(),
            image: "/iphone.png".to_string()
        },
        Product {
            id: 5,
            name: "iphone11".to_string(),
            price: 888.99,
            description: "Across the ages, products meet all your needs.".to_string(),
            image: "/iphone11.png".to_string()
        },
        Product {
            id: 6,
            name: "iphone13".to_string(),
            price: 1124.99,
            description: "Impressive photo capabilities, high-definition images.".to_string(),
            image: "/iphone13.png".to_string()
        },
        Product {
            id: 7,
            name: "iphone16".to_string(),
            price: 1933.99,
            description: "Change the world again.".to_string(),
            image: "/iphone16.png".to_string()
        },
        Product {
            id: 8,
            name: "RiceRollMachine".to_string(),
            price: 73.99,
            description: "A kitchen product that brings convenience to life.".to_string(),
            image: "/RiceRollMachine.png".to_string()
        },
        Product {
            id: 9,
            name: "TheStooges".to_string(),
            price: 34.99,
            description: "niche music.".to_string(),
            image: "/TheStooges.png".to_string()
        },
        Product {
            id: 10,
            name: "MirandaLambert".to_string(),
            price: 55.99,
            description: "SuperStar.".to_string(),
            image: "/MirandaLambert.png".to_string()
        }
    ]
}
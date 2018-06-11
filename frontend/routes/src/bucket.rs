
use routing::*;
use identifiers::bucket::BucketUuid;

#[derive(Debug, PartialEq, Clone)]
pub enum BucketRoute {
    BucketList,
    Bucket{bucket_uuid: BucketUuid},
    Create
}

impl Default for BucketRoute {
    fn default() -> Self {
        BucketRoute::BucketList
    }
}

impl Router for BucketRoute {
    fn to_route(&self) -> RouteInfo {
        use self::BucketRoute::*;
        match *self {
            BucketList => RouteInfo::parse("/").unwrap(),
            Bucket{bucket_uuid} => RouteInfo::parse(&format!("/{}", bucket_uuid)).unwrap(),
            Create => RouteInfo::parse("/create").unwrap()
        }
    }
    fn from_route(route: &mut RouteInfo) -> Option<Self> {
        use self::BucketRoute::*;
        if let Some(RouteSection::Node { segment }) = route.next() {
            if let Ok(bucket_uuid) = BucketUuid::parse_str(&segment) {
                Some(Bucket{bucket_uuid})
            } else if segment == "create" {
                Some(Create)
            } else {
                Some(BucketList)
            }
        } else {
            None
        }
    }
}
struct Link{
    src : Box<Link>,  //pageID
    nlink: u32,       //웹페이지에서 다른 웹페이지로 나가는 URL의 count
    dest: Box<Link>,  //다른 웹페이지로 나가는 URL들의 pageID
}

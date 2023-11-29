create table price (
  id int primary key auto_increment,
  stock_id int not null,
  transaction_date datetime not null,
  open float(2) not null,
  high float(2) not null,
  low  float(2) not null,
  close float(2) not null,
  volume int not null,
  FOREIGN KEY (stock_id) references stock(id)
);

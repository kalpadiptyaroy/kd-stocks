create table stock (
  id int primary key auto_increment,
	nse_symbol varchar(20) not null,
  bse_symbol varchar(20) not null,
  name varchar(75) not null
);

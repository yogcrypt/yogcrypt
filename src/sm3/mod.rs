static IV: [u32; 8] = [0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e];

fn T_j(j: u32) ->u32 
{
	assert!(j>=0 && j<=63);  

	if j<=15
	{
		0x79CC4519
	}
	else 
	{
		0x7A879D8A
	}
}

fn FF_j(X: u32, Y: u32, Z: u32, j:u32) -> u32
{
	assert!(j>=0 && j<=63);

	if j<=15
	{
		X ^ Y ^ Z
	}
	else 
	{
		(X & Y) | (X & Z) | (Y & Z)
	}
}

fn GG_j(X: u32, Y: u32, Z: u32, j: u32) -> u32
{
	assert!(j>=0 && j<=63);

	if j<=15
	{
		X ^ Y ^ Z
	}
	else 
	{
		(X & Y) | ((!X) & Z)
	}	
}

fn P_0(X: u32) -> u32
{
	X ^ X.rotate_left(9) ^ X.rotate_left(17)
}

fn P_1(X: u32) -> u32
{
	X ^ X.rotate_left(15) ^ X.rotate_left(23)
}

pub struct Sm3Cryptor
{

}
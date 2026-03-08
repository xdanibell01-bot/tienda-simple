use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMQHgqE3s7P");

#[program]
pub mod tienda_simple {
    use super::*;

    // Crear tienda
    pub fn crear_tienda(ctx: Context<CrearTienda>, nombre: String) -> Result<()> {

        require!(nombre.len() <= 50, ErrorCodigo::NombreMuyLargo);

        let tienda = &mut ctx.accounts.tienda;

        tienda.owner = ctx.accounts.owner.key();
        tienda.nombre = nombre;
        tienda.productos = Vec::new();
        tienda.bump = ctx.bumps.tienda;

        Ok(())
    }

    // Agregar tenis
    pub fn agregar_tenis(
        ctx: Context<ModificarTienda>,
        nombre: String,
        precio: u64,
        stock: u16,
    ) -> Result<()> {

        require!(nombre.len() <= 50, ErrorCodigo::NombreMuyLargo);

        let tienda = &mut ctx.accounts.tienda;

        require!(
            tienda.owner == ctx.accounts.owner.key(),
            ErrorCodigo::NoEresOwner
        );

        require!(
            tienda.productos.len() < 20,
            ErrorCodigo::LimiteProductos
        );

        let nuevo_tenis = Tenis {
            nombre,
            precio,
            stock,
        };

        tienda.productos.push(nuevo_tenis);

        Ok(())
    }

    // Ver tenis
    pub fn ver_tenis(ctx: Context<ModificarTienda>) -> Result<()> {
        msg!("Lista de tenis: {:#?}", ctx.accounts.tienda.productos);
        Ok(())
    }

    // Comprar tenis
    pub fn comprar_tenis(
        ctx: Context<ModificarTienda>,
        nombre: String,
    ) -> Result<()> {

        let tienda = &mut ctx.accounts.tienda;

        for i in 0..tienda.productos.len() {

            if tienda.productos[i].nombre == nombre {

                require!(
                    tienda.productos[i].stock > 0,
                    ErrorCodigo::SinStock
                );

                tienda.productos[i].stock -= 1;

                msg!("Compra exitosa!");
                return Ok(());
            }
        }

        Err(ErrorCodigo::ProductoNoExiste.into())
    }
}

#[error_code]
pub enum ErrorCodigo {

    #[msg("No eres el dueño de la tienda")]
    NoEresOwner,

    #[msg("No hay stock disponible")]
    SinStock,

    #[msg("El producto no existe")]
    ProductoNoExiste,

    #[msg("Nombre demasiado largo")]
    NombreMuyLargo,

    #[msg("Limite de productos alcanzado")]
    LimiteProductos,
}

#[account]
pub struct Tienda {

    pub owner: Pubkey,        // 32
    pub nombre: String,       // 4 + 50
    pub productos: Vec<Tenis>,// 4 + (max productos)
    pub bump: u8,             // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Tenis {

    pub nombre: String, // 4 + 50
    pub precio: u64,    // 8
    pub stock: u16,     // 2
}

#[derive(Accounts)]
pub struct CrearTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,

        // 8 discriminator
        // 32 owner
        // 4 + 50 nombre
        // 4 vec length
        // 20 productos max
        // cada tenis = 4 + 50 + 8 + 2 = 64 aprox
        // 20 * 64 = 1280
        // + bump
        space = 8 + 32 + 54 + 4 + (20 * 64) + 1,

        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"tienda", owner.key().as_ref()],
        bump = tienda.bump
    )]
    pub tienda: Account<'info, Tienda>,
}

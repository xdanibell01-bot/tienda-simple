use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod tienda_simple {
    use super::*;

    // 1️⃣ Crear tienda
    pub fn crear_tienda(ctx: Context<CrearTienda>, nombre: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        tienda.owner = ctx.accounts.owner.key();
        tienda.nombre = nombre;
        tienda.productos = Vec::new();
        Ok(())
    }

    // 2️⃣ Agregar tenis
    pub fn agregar_tenis(
        ctx: Context<ModificarTienda>,
        nombre: String,
        precio: u64,
        stock: u16,
    ) -> Result<()> {

        let tienda = &mut ctx.accounts.tienda;

        require!(
            tienda.owner == ctx.accounts.owner.key(),
            ErrorCodigo::NoEresOwner
        );

        let nuevo_tenis = Tenis {
            nombre,
            precio,
            stock,
        };

        tienda.productos.push(nuevo_tenis);
        Ok(())
    }

    // 3️⃣ Ver tenis
    pub fn ver_tenis(ctx: Context<ModificarTienda>) -> Result<()> {
        msg!("Lista de tenis: {:#?}", ctx.accounts.tienda.productos);
        Ok(())
    }

    // 4️⃣ Comprar tenis
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

//////////////////////////////////// ERRORES ////////////////////////////////////

#[error_code]
pub enum ErrorCodigo {
    #[msg("No eres el dueño de la tienda")]
    NoEresOwner,

    #[msg("No hay stock disponible")]
    SinStock,

    #[msg("El producto no existe")]
    ProductoNoExiste,
}

//////////////////////////////////// CUENTA TIENDA ///////////////////////////////

#[account]
pub struct Tienda {
    pub owner: Pubkey,
    pub nombre: String,
    pub productos: Vec<Tenis>,
}

//////////////////////////////////// STRUCT TENIS ////////////////////////////////

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Tenis {
    pub nombre: String,
    pub precio: u64,
    pub stock: u16,
}

//////////////////////////////////// CONTEXTOS ///////////////////////////////////

#[derive(Accounts)]
pub struct CrearTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(init, payer = owner, space = 1000)]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}

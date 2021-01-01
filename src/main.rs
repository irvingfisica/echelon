use nalgebra::{Matrix4x5};
use nalgebra::{DefaultAllocator,allocator::Allocator};

fn main() {
    let mut m = Matrix4x5::new(
        1.0, 7.0, 4.0, 3.0, 5.0,
        0.0, 0.0, 1.0, 7.0, 6.0,
        0.0, 0.0, 1.0, 5.0, 3.0,
        0.0, 0.0, 0.0, 0.0, 1.0);

    println!("{}",m);

    let fils = m.nrows();

    for i in 0..fils {
        for j in (i+1)..fils{
            row_addition(&mut m, i, j);
        }
    }

    for i in 0..fils {
        row_scaling(&mut m, i);
    }

    println!("{}",m);
}

fn row_scaling<R, C, S>(
    matriz: &mut nalgebra::Matrix<f64,R,C,S>,
    index: usize)
where
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    S: nalgebra::storage::StorageMut<f64,R,C>,
    DefaultAllocator: Allocator<f64,nalgebra::U1,C>
{
    let fila = matriz.row(index);
    let nz_ind = fila.iter().position(|&ele| ele != 0.0);

    let nz_ind = match nz_ind {
        Some(v) => v,
        None => {
            return ()
        }
    };

    let v = fila[nz_ind];

    let nrow = fila.map(|ele| ele/v);
    matriz.set_row(index,&nrow);

    ()
}

fn row_swapper<R, C, S>(
    matriz: &mut nalgebra::Matrix<f64,R,C,S>,
    up_index: usize, 
    lo_index: usize)
where 
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    S: nalgebra::storage::StorageMut<f64,R,C>,
    DefaultAllocator: Allocator<f64,nalgebra::U1,C>
{
    let upper = matriz.row(up_index);
    let lower = matriz.row(lo_index);

    let nz_uind = upper.iter().position(|&ele| ele != 0.0);

    let nz_uind = match nz_uind {
        Some(v) => v,
        None => {
            matriz.swap_rows(up_index,lo_index);
            return ()
        }
    };

    let nz_lind = lower.iter().position(|&ele| ele != 0.0);

    let nz_lind = match nz_lind {
        Some(v) => v,
        None => {
            return ()
        }
    };

    if nz_uind > nz_lind {
        matriz.swap_rows(up_index, lo_index);
        return ()
    }

}

fn row_addition<R, C, S>(
    matriz: &mut nalgebra::Matrix<f64,R,C,S>,
    up_index: usize, 
    lo_index: usize)
where 
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    S: nalgebra::storage::StorageMut<f64,R,C>,
    DefaultAllocator: Allocator<f64,nalgebra::U1,C>
{
    let upper = matriz.row(up_index);
    let lower = matriz.row(lo_index);

    let nz_uind = upper.iter().position(|&ele| ele != 0.0);
    let nz_uind = match nz_uind {
        Some(v) => v,
        None => {
            return ()
        }
    };

    let nz_lind = lower.iter().position(|&ele| ele != 0.0);
    let nz_lind = match nz_lind {
        Some(v) => v,
        None => {
            return ()
        }
    };

    if nz_uind == nz_lind {
        let lv = lower[nz_lind];
        let uv = upper[nz_uind];
        let nrow = lower - upper.map(|ele| ele*lv/uv);
        matriz.set_row(lo_index,&nrow);       
    }

    ()
}
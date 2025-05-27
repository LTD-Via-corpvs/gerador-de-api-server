const BaseController = ({ save, getOne, getAll, getTotalObjects, update, remove, include }) => {

    const create = async (req, res) => {
        try {
            const obj = await save(req.body)
            res.status(201).json(obj);
        } catch (error) {
            console.error('Erro ao criar usuário:', error);
            res.status(500).json({ error: 'Erro ao criar objeto!' });
        }
    }
    const readOne = async (req, res) => {
        const { id } = req.params
        try {
            const obj = await getOne(parseInt(id), include)
            res.status(200).json(obj)
        } catch (error) {
            console.log(error)
            res.status(500).json({ error: 'Erro ao buscar objeto!' })
        }
    }
    const readAll = async (req, res) => {
        try {
            //TODO: Remover isso daqui e criar um ReadPage
            const { page = 1, pageSize = 10 } = req.query
            const obj = await getAll(parseInt(page), parseInt(pageSize), include)

            const totalObjects = await getTotalObjects()
            const totalPages = Math.ceil(totalObjects / parseInt(pageSize))

            const paginationInfo = {
                page: parseInt(page),
                totalPages: totalPages,
                rowCount: totalObjects,
            }
            res.status(200).json({ data: obj, pagination: paginationInfo });
        } catch (error) {
            console.log(error)
            res.status(500).json({ error: 'Erro ao buscar todos os objetos' })
        }
    }
    const updateObj = async (req, res) => {
        const { id } = req.params
        try {
            const obj = await update(parseInt(id), req.body)
            res.status(201).json(obj)
        } catch (error) {
            console.log(error)
            res.status(500).json({ error: 'Erro ao atualizar objeto!' })
        }
    }
    const removeObj = async (req, res) => {
        const { id } = req.params;
        try {
            const obj = await remove(parseInt(id))
            res.status(200).json(obj);
        } catch (error) {
            console.log(error);
            res.status(500).json({ error: 'Erro ao excluir objeto no banco de dados!' });
        }
    }

    return {
        create,
        readOne,
        readAll,
        updateObj,
        removeObj
    }
}

export default BaseController
export { BaseController }
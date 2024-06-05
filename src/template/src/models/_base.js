import { prisma } from '../database/index.js'

const BaseModel = ({ model = '', junctionTableModel = null }) => {

    const getTotalObjects = async () => {
        try {
            const count = await prisma[model].count();
            return count
        } catch (error) {
            console.log(error)
            throw new Error('Erro ao obter total de objetos!')
        }
    }
    const noRelations = async (id, params) => {
        if (id === "create") {
            const obj = await prisma[model].create({
                data: {
                    ...params
                }
            })
            return obj
        }
        if (id !== "create") {
            const obj = await prisma[model].update({
                where: {
                    id: id
                },
                data: {
                    ...params
                }
            })
            return obj
        }
    }
    const oneToManyRelation = async (id, params) => {
        if ('type' in params) delete params.type
        if (id === "create") {
            const data = params
            const obj = await prisma[model].create({
                data: {
                    ...data,
                }
            })
            return obj
        }
        if (id !== "create") {
            const data = params
            const obj = await prisma.funcionario.update({
                where: { id: id },
                data: { ...data },
            })
            return obj
        }
    }
    const manyToManyRelation = async (id, params) => {
        if ('type' in params) delete params.type
        if (id === "create") {
            const { modelRelated, associationTable, connect, ...data } = params
            const dataConnect = connect.map(id => ({ id }))

            const obj = await prisma[model].create({
                data: {
                    ...data,
                    [associationTable]: {
                        create: dataConnect.map(item => ({
                            [modelRelated]: {
                                connect: item
                            }
                        })),
                    }
                }
            })
            return obj
        }
        if (id !== "create") {
            if ('type' in params) delete params.type
            const { modelId, modelField, modelName, connectId, connectField, ...data } = params
            //Getting all records in the junction table
            const objs = await prisma[model].findMany({
                where: {
                    [modelField]: Number(modelId)
                }
            })
            //Updating the model data
            await prisma[modelName].update({
                where: { id: modelId },
                data: { ...data }
            })

            const arrResponse = []
            const existingIds = objs.map(obj => obj.id)
            //Updating or creating
            for (let ix = 0; ix < connectId.length; ix++) {
                if (ix < existingIds.length) {
                    // Updating existing records
                    const objUpdated = await prisma[model].update({
                        where: {
                            id: existingIds[ix]
                        },
                        data: {
                            [connectField]: connectId[ix]
                        }
                    })
                    if (objUpdated.id) delete objUpdated.id
                    arrResponse.push(objUpdated)
                } else {
                    // Creating new records
                    const objCreated = await prisma[model].create({
                        data: {
                            [modelField]: Number(modelId),
                            [connectField]: connectId[ix]
                        }
                    })
                    if (objCreated.id) delete objCreated.id
                    arrResponse.push(objCreated)
                }
            }
            return { data: arrResponse, message: "Update concluído com sucesso!" }
        }
    }

    //Services
    const save = async (params) => {
        try {
            const newobj = await params.type === "N-N" ? manyToManyRelation("create", params) : params.type === "1-N" ? oneToManyRelation("create", params) : noRelations("create", params)
            return newobj
        } catch (error) {
            console.log(error)
            throw new Error('Erro ao salvar objeto no banco de dados!')
        }
    }
    const getOne = async (id, include) => {
        try {
            const obj = await prisma[model].findFirst({
                where: {
                    id: id
                },
                include: include
            })
            if (obj) return obj
            else return { message: 'Objeto não encontrado no banco de dados!' }
        } catch (error) {
            console.log(error)
            throw new Error('Erro ao ler objeto no banco de dados!')
        }
    }
    const getAll = async (page, pageSize, include) => {
        //TODO: IF TIVER NADA
        try {
            const skip = (page - 1) * pageSize
            const obj = await prisma[model].findMany({
                take: pageSize,
                skip: skip,
                include: include
            })
            if (obj) return obj
            else return { message: 'Não há registros de objetos no banco de dados' }
        } catch (error) {
            console.log(error)
            throw new Error('Erro ao ler objetos no banco de dados!')
        }
    }
    const update = async (id, params) => {
        try {
            const obj = await params.type === "N-N" ? manyToManyRelation(id, params) : params.type === "1-N" ? oneToManyRelation(id, params) : noRelations(id, params)
            return obj
        } catch (error) {
            console.log(error)
            throw new Error('Erro ao atualizar objeto no banco de dados!')
        }
    }
    const remove = async (id) => {
        //TODO: DEIXAR ESSA LÓGICA MAIS FÁCIL
        try {
            if (junctionTableModel === null) {
                const obj = await prisma[model].delete({
                    where: {
                        id: id
                    }
                })
                return obj
            }
            if (junctionTableModel !== null) {
                const idFk = `id_${model}`
                await prisma[junctionTableModel].deleteMany({
                    where: {
                        [idFk]: id
                    }
                })
                const obj = await prisma[model].delete({
                    where: {
                        id: id
                    }
                })
                return obj
            }
        } catch (error) {
            console.log(error)
            throw new Error('Erro ao deleter objeto no banco de dados!')
        }
    }

    return {
        save,
        getOne,
        getAll,
        getTotalObjects,
        update,
        remove,
    }
}
export default BaseModel
export { BaseModel }
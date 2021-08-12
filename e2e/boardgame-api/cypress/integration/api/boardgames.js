/// <reference types="cypress" />

context('ボードゲームの一覧を取得することができる', () => {
  it('ボードゲームの一覧を取得する', () => {
    cy.request({
      url: 'localhost:21001/v1/boardgames',
      method: 'GET',
    })
    .then((response) => {
      expect(response).property('status').to.equal(200)
      expect(response).property('body').to.have.property('length').and.be.oneOf([500, 501])
      expect(response).to.include.keys('headers', 'duration')
    })
  })
})
